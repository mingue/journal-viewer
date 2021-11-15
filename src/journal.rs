use crate::libsdjournal::*;
use crate::query_builder::QueryBuilder;
use bitflags::bitflags;
use libc::{c_char, c_void, size_t};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

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

    pub fn open(open_flags: OpenFlags) -> Result<Journal, i32> {
        let ret: libc::c_int;
        let mut journal = Journal::new();

        unsafe {
            ret = sd_journal_open(&mut journal.ptr, open_flags.bits());
        }

        if ret == 0 {
            return Ok(journal);
        }

        Err(ret)
    }

    pub fn get_logs(&self) -> Result<Vec<HashMap<&str, String>>, i32> {
        self.get_logs_internal()
    }

    pub fn query_logs(&self, qb: &QueryBuilder) -> Result<Vec<HashMap<&str, String>>, i32> {
        let ret: libc::c_int = 0;

        self.apply_pid_filter(qb);
        self.apply_minimum_priority(qb);
        self.apply_unit(qb);
        self.apply_slice(qb);

        self.get_logs_internal()
    }

    pub fn close(&mut self) {
        unsafe {
            sd_journal_close(self.ptr);
        }
    }

    fn get_logs_internal(&self) -> Result<Vec<HashMap<&str, String>>, i32> {
        let mut logs = Vec::new();

        for _i in 0..11 {
            let ret: libc::c_int;

            unsafe {
                ret = sd_journal_next(self.ptr);
            }

            if ret == 0 {
                break;
            }

            if ret < 0 {
                return Err(ret);
            }

            let fields = ["MESSAGE", "PRIORITY", "_PID", "_COMM", "_UID", "_GID"];
            let mut dic = HashMap::new();

            for field in fields {
                match self.get_field(field) {
                    Ok(data) => {
                        dic.insert(field, data);
                    }
                    Err(_) => {}
                }
            }

            logs.push(dic);
        }
        Ok(logs)
    }

    fn get_field(&self, field: &str) -> Result<String, i32> {
        let mut data: *const c_void = std::ptr::null_mut();
        let mut length: size_t = 0;
        let c_field = CString::new(field).expect("CString failed");
        let ret: libc::c_int;
        unsafe {
            ret = sd_journal_get_data(self.ptr, c_field.as_ptr(), &mut data, &mut length);
        }
        if ret < 0 {
            return Err(ret);
        }
        let result = unsafe {
            match CStr::from_ptr(data as *mut c_char).to_str() {
                Ok(s) => {
                    let s = String::from(s);
                    let remove = format!("{}=", field);
                    if let Some(value) = s.strip_prefix(&remove) {
                        return Ok(value.to_string());
                    }
                    Ok(s)
                }
                Err(_) => Err(-1),
            }
        };

        result
    }

    fn apply_pid_filter(&self, qb: &QueryBuilder) {
        if qb.pid > 0 {
            unsafe {
                let data = CString::new(format!("_PID={}", qb.pid)).expect("Could not set pid");
                sd_journal_add_match(self.ptr, data.as_ptr(), 0usize);
            }
        }
    }

    fn apply_minimum_priority(&self, qb: &QueryBuilder) {
        unsafe {
            let data = CString::new(format!("PRIORITY={}", qb.minimum_priority))
                .expect("Could not set priority");
            sd_journal_add_match(self.ptr, data.as_ptr(), 0usize);
        }
    }

    fn apply_unit(&self, qb: &QueryBuilder) {
        if qb.unit != String::new() {
            unsafe {
                let data =
                    CString::new(format!("_SYSTEMD_UNIT={}", qb.unit)).expect("Could not set unit");
                sd_journal_add_match(self.ptr, data.as_ptr(), 0usize);
            }
        }
    }

    fn apply_slice(&self, qb: &QueryBuilder) {
        if qb.slice != String::new() {
            unsafe {
                let data = CString::new(format!("_SYSTEMD_SLICE={}", qb.slice))
                    .expect("Could not set slice");
                sd_journal_add_match(self.ptr, data.as_ptr(), 0usize);
            }
        }
    }
}
