use crate::journal_entries::JournalEntries;
use crate::libsdjournal::*;
use crate::query_builder::{fields, QueryBuilder};
use bitflags::bitflags;
use libc::c_void;
use std::vec;

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

#[derive(Debug)]
pub struct Journal {
    ptr: *const c_void,
}

impl Journal {
    fn new() -> Journal {
        let sd_journal = Journal {
            ptr: std::ptr::null_mut(),
        };

        sd_journal
    }

    pub fn open(open_flags: OpenFlags) -> Result<Journal, JournalError> {
        let mut journal = Journal::new();
        sd_journal_open(&mut journal.ptr, open_flags.bits())?;

        Ok(journal)
    }

    pub fn get_logs(&self) -> Result<JournalEntries, JournalError> {
        let qb = QueryBuilder::default();

        self.get_logs_internal(&qb)
    }

    pub fn query_logs(&self, qb: &QueryBuilder) -> Result<JournalEntries, JournalError> {
        self.get_logs_internal(qb)
    }

    fn get_logs_internal(&self, qb: &QueryBuilder) -> Result<JournalEntries, JournalError> {
        self.apply_pid_filter(qb);
        self.apply_minimum_priority(qb);
        self.apply_unit(qb);
        self.apply_slice(qb);
        self.apply_boot_id(qb);

        let mut journal_entries = JournalEntries::new();

        for field in qb.fields.iter() {
            journal_entries.headers.push((*field).to_string())
        }

        sd_journal_seek_tail(self.ptr)?;

        if qb.skip > 0 {
            sd_journal_previous_skip(self.ptr, qb.skip);
        }

        let mut count: u64 = 0;

        loop {
            let more = sd_journal_previous(self.ptr)?;
            count+=1;

            if !more || (qb.limit > 0 && count >= qb.limit){
                break;
            }

            let mut row: Vec<String> = vec![];

            for field in qb.fields.iter() {
                match self.get_field(*field) {
                    Ok(data) => {
                        row.push(data);
                    }
                    Err(e) => {
                        row.push(String::new());
                        warn!("Could not find the field {}", e);
                    }
                }
            }

            journal_entries.rows.push(row);
        }

        Ok(journal_entries)
    }

    fn get_field(&self, field: &str) -> Result<String, JournalError> {
        sd_journal_get_data(self.ptr, field)
    }

    fn apply_pid_filter(&self, qb: &QueryBuilder) {
        if qb.pid > 0 {
            let query = String::from(format!("{}={}", fields::PID, qb.pid));
            if let Err(e) = sd_journal_add_match(self.ptr, query) {
                warn!("Could not apply filter {}", e);
            }
        }
    }

    fn apply_minimum_priority(&self, qb: &QueryBuilder) {
        let query = String::from(format!("{}={}", fields::PRIORITY, qb.minimum_priority));
        if let Err(e) = sd_journal_add_match(self.ptr, query) {
            warn!("Could not apply filter {}", e);
        }
    }

    fn apply_unit(&self, qb: &QueryBuilder) {
        if !qb.unit.is_empty() {
            let query = String::from(format!("{}={}", fields::SYSTEMD_UNIT, qb.unit));
            if let Err(e) = sd_journal_add_match(self.ptr, query) {
                warn!("Could not apply filter {}", e);
            }
        }
    }

    fn apply_slice(&self, qb: &QueryBuilder) {
        if !qb.slice.is_empty() {
            let query = String::from(format!("{}={}", fields::SYSTEMD_SLICE, qb.slice));
            if let Err(e) = sd_journal_add_match(self.ptr, query) {
                warn!("Could not apply filter {}", e);
            }
        }
    }

    fn apply_boot_id(&self, qb: &QueryBuilder) {
        if !qb.boot_id.is_empty() {
            let query = String::from(format!("{}={}", fields::BOOT_ID, qb.boot_id));
            if let Err(e) = sd_journal_add_match(self.ptr, query) {
                warn!("Could not apply filter {}", e);
            }
        }
    }
}

impl Drop for Journal {
    fn drop(&mut self) {
        sd_journal_close(self.ptr);
    }
}
