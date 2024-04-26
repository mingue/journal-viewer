use crate::boot::Boot;
use crate::journal_entries::JournalEntries;
use crate::journal_entries::JournalEntry;
use crate::journal_fields;
use crate::libsdjournal::*;
use crate::query::Query;
use crate::query_builder::QueryBuilder;
use crate::unit::Unit;
use bitflags::bitflags;
use libc::c_void;
use std::process::Command;

bitflags! {
    #[repr(C)]
    pub struct OpenFlags: u32 {
        /// Only files generated on the local machine
        const SD_JOURNAL_LOCAL_ONLY = 1 << 0;
        /// Only volatile journal files excluding persisted
        const SD_JOURNAL_RUNTIME_ONLY = 1 << 1;
        /// System services and the Kernel
        const SD_JOURNAL_SYSTEM = 1 << 2;
        /// Current user
        const SD_JOURNAL_CURRENT_USER = 1 << 3;
    }
}

pub const INIT_UNIT: &str = "Init (Systemd)";

#[derive(Debug)]
pub struct Journal {
    ptr: *mut c_void,
}

impl Journal {
    fn new() -> Journal {
        Journal {
            ptr: std::ptr::null_mut(),
        }
    }

    pub fn open(open_flags: OpenFlags) -> Result<Journal, JournalError> {
        let mut journal = Journal::new();
        sd_journal_open(&mut journal.ptr, open_flags.bits())?;

        Ok(journal)
    }

    pub fn get_logs(&self) -> Result<JournalEntries, JournalError> {
        let q = QueryBuilder::default().build();

        self.get_logs_internal(&q)
    }

    pub fn query_logs(&self, q: &Query) -> Result<JournalEntries, JournalError> {
        self.get_logs_internal(q)
    }

    fn get_logs_internal(&self, q: &Query) -> Result<JournalEntries, JournalError> {
        sd_journal_flush_matches(self.ptr);

        self.apply_pid_filter(q);
        self.apply_minimum_priority(q);
        self.apply_units(q);
        self.apply_slice(q);
        self.apply_boot_ids(q);
        self.apply_transports_filter(q);

        let mut journal_entries = JournalEntries::new(q.limit as usize);

        for field in q.fields.iter() {
            journal_entries.headers.push((*field).to_string())
        }

        if q.reset_position {
            sd_journal_seek_tail(self.ptr)?;
        }

        if q.reset_position && q.date_less_than > 0 {
            sd_journal_seek_realtime_usec(self.ptr, q.date_less_than)?;
        }

        let mut count: u64 = 0;
        let mut last_timestamp: u64 = 0;

        loop {
            let more = sd_journal_previous(self.ptr)?;

            if !more {
                debug!("No more entries");
                break;
            }

            if let Ok(updated_timestamp) = self.get_field(journal_fields::SOURCE_REALTIME_TIMESTAMP)
            {
                last_timestamp = updated_timestamp.parse().unwrap();
                trace!(
                    "Last timestamp {:?}",
                    chrono::DateTime::from_timestamp_micros(last_timestamp.try_into().unwrap())
                )
            }

            if !q.quick_search.is_empty() {
                if let Ok(message) = self.get_field(journal_fields::MESSAGE) {
                    if !message.to_lowercase().contains(&q.quick_search) {
                        continue;
                    }
                }
            }

            if q.limit > 0 && count >= q.limit {
                debug!("Reached limit of {}", q.limit);
                break;
            }

            if q.date_more_than > 0 && q.date_more_than >= last_timestamp {
                debug!("Reached epoch time of {}", q.date_more_than);
                break;
            }

            let mut row: Vec<String> = Vec::with_capacity(q.fields.len());

            for field in q.fields.iter() {
                match field.as_str() {
                    "__REALTIME" => {
                        let mut realtime: u64 = 0;
                        match sd_journal_get_realtime_usec(self.ptr, &mut realtime) {
                            Ok(()) => row.push(realtime.to_string()),
                            Err(JournalError::Internal(e)) => {
                                row.push(String::new());
                                warn!("Could not get realtime field, error: {}", e);
                            }
                            Err(JournalError::EndOfFile) => {
                                panic!("should not return end of file")
                            }
                        }
                    }
                    _ => match self.get_field(field) {
                        Ok(data) => {
                            row.push(data);
                        }
                        Err(e) => {
                            row.push(String::new());
                            warn!("Could not find the field: {}, JournalError: {}", &field, e);
                        }
                    },
                }
            }

            journal_entries.rows.push(row);
            count += 1;
        }

        Ok(journal_entries)
    }

    pub fn get_full_entry(&self, timestamp: u64) -> Result<JournalEntry, JournalError> {
        sd_journal_seek_realtime_usec(self.ptr, timestamp)?;

        let more = sd_journal_previous(self.ptr)?;

        if !more {
            error!("Entry not found by the timestamp");
            return Err(JournalError::Internal(0));
        }

        let mut entry = JournalEntry::new();

        loop {
            match sd_journal_enumerate_data(self.ptr) {
                Ok(x) => {
                    if !x.0.is_empty() {
                        entry.headers.push(x.0);
                        entry.values.push(x.1);
                    }
                }
                Err(err) => match err {
                    JournalError::EndOfFile => break, // If we reach the end just ignore
                    _ => return Err(err),             // Other type of error, return it
                },
            }
        }

        Ok(entry)
    }

    fn get_field(&self, field: &str) -> Result<String, JournalError> {
        sd_journal_get_data(self.ptr, field)
    }

    fn apply_pid_filter(&self, q: &Query) {
        if q.pid > 0 {
            let query = format!("{}={}", journal_fields::PID, q.pid);
            if let Err(e) = sd_journal_add_match(self.ptr, query) {
                warn!("Could not apply filter {}", e);
            }
        }
    }

    fn apply_transports_filter(&self, q: &Query) {
        if !q.transports.is_empty() {
            for transport in q.transports.iter() {
                let query = format!("{}={}", journal_fields::TRANSPORT, transport);
                if let Err(e) = sd_journal_add_match(self.ptr, query) {
                    warn!("Could not apply filter {}", e);
                }
            }
        }
    }

    fn apply_minimum_priority(&self, q: &Query) {
        for p in 0..=q.minimum_priority {
            let query = format!("{}={}", journal_fields::PRIORITY, p);
            if let Err(e) = sd_journal_add_match(self.ptr, query) {
                warn!("Could not apply filter {}", e);
            }
        }
    }

    fn apply_units(&self, q: &Query) {
        if !q.units.is_empty() {
            for unit in q.units.iter() {
                let query = format!("{}={}", journal_fields::UNIT_FILTER, unit);
                if let Err(e) = sd_journal_add_match(self.ptr, query) {
                    warn!("Could not apply filter {}", e);
                }
            }
        }
    }

    fn apply_slice(&self, q: &Query) {
        if !q.slice.is_empty() {
            let query = format!("{}={}", journal_fields::SYSTEMD_SLICE, q.slice);
            if let Err(e) = sd_journal_add_match(self.ptr, query) {
                warn!("Could not apply filter {}", e);
            }
        }
    }

    fn apply_boot_ids(&self, q: &Query) {
        if !q.boot_ids.is_empty() {
            for boot_id in q.boot_ids.iter() {
                let query = format!("{}={}", journal_fields::BOOT_ID, boot_id);
                if let Err(e) = sd_journal_add_match(self.ptr, query) {
                    warn!("Could not apply filter {}", e);
                }
            }
        }
    }

    pub fn list_services() -> Vec<Unit> {
        let output = Command::new("systemctl")
            .arg("list-unit-files")
            .arg("*.service")
            .arg("-o")
            .arg("json")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();

        let mut units: Vec<Unit> = serde_json::from_str(&stdout).unwrap();
        units.insert(
            0,
            Unit {
                unit_file: INIT_UNIT.into(),
                state: "".into(),
                preset: Option::None,
            },
        );

        units
    }

    pub fn list_boots() -> Vec<Boot> {
        let output = Command::new("journalctl")
            .arg("--list-boots")
            .arg("-r")
            .arg("-o")
            .arg("json")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();

        serde_json::from_str(&stdout).unwrap()
    }
}

impl Drop for Journal {
    fn drop(&mut self) {
        warn!("Dropping the journal");
        sd_journal_close(self.ptr);
    }
}

unsafe impl Send for Journal {}
