use crate::libsdjournal::*;
use crate::query_builder::QueryBuilder;
use bitflags::bitflags;
use libc::c_void;
use std::collections::HashMap;

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

    pub fn get_logs(&self) -> Result<Vec<HashMap<&str, String>>, JournalError> {
        self.get_logs_internal()
    }

    pub fn query_logs(
        &self,
        qb: &QueryBuilder,
    ) -> Result<Vec<HashMap<&str, String>>, JournalError> {
        self.apply_pid_filter(qb);
        self.apply_minimum_priority(qb);
        self.apply_unit(qb);
        self.apply_slice(qb);

        self.get_logs_internal()
    }

    fn get_logs_internal(&self) -> Result<Vec<HashMap<&str, String>>, JournalError> {
        let mut logs = Vec::new();

        for _i in 0..11 {
            let more = sd_journal_next(self.ptr)?;

            if !more {
                break;
            }

            let fields = ["MESSAGE", "PRIORITY", "_PID", "_COMM", "_UID", "_GID"];
            let mut dic = HashMap::new();

            for field in fields {
                match self.get_field(field) {
                    Ok(data) => {
                        dic.insert(field, data);
                    }
                    Err(_) => {} // If we can get one field for a log we ignore it
                }
            }

            logs.push(dic);
        }
        Ok(logs)
    }

    fn get_field(&self, field: &str) -> Result<String, JournalError> {
        sd_journal_get_data(self.ptr, field)
    }

    fn apply_pid_filter(&self, qb: &QueryBuilder) {
        if qb.pid > 0 {
            let query = String::from(format!("_PID:{}", qb.pid));
            if let Err(e) = sd_journal_add_match(self.ptr, query) {
                warn!("Could not apply pid filter {}", e);
            }
        }
    }

    fn apply_minimum_priority(&self, qb: &QueryBuilder) {
        let query = String::from(format!("PRIORITY={}", qb.minimum_priority));
        if let Err(e) = sd_journal_add_match(self.ptr, query) {
            warn!("Could not apply pid filter {}", e);
        }
    }

    fn apply_unit(&self, qb: &QueryBuilder) {
        if qb.unit != String::new() {
            let query = String::from(format!("_SYSTEMD_UNIT={}", qb.unit));
            if let Err(e) = sd_journal_add_match(self.ptr, query) {
                warn!("Could not apply pid filter {}", e);
            }
        }
    }

    fn apply_slice(&self, qb: &QueryBuilder) {
        if qb.slice != String::new() {
            let query = String::from(format!("_SYSTEMD_SLICE={}", qb.slice));
            if let Err(e) = sd_journal_add_match(self.ptr, query) {
                warn!("Could not apply pid filter {}", e);
            }
        }
    }
}

impl Drop for Journal {
    fn drop(&mut self) {
        sd_journal_close(self.ptr);
    }
}
