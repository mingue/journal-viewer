use crate::libsdjournal::*;
use bitflags::bitflags;
use libc::{c_char, c_void, size_t};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

bitflags! {
    #[repr(C)]
    pub struct OpenFlags: u32 {
        const SD_JOURNAL_LOCAL_ONLY = 1 << 0;
        const SD_JOURNAL_RUNTIME_ONLY = 1 << 1;
        const SD_JOURNAL_SYSTEM = 1 << 2;
        const SD_JOURNAL_CURRENT_USER = 1 << 3;
        const SD_JOURNAL_OS_ROOT = 1 << 4;
        const SD_JOURNAL_ALL_NAMESPACES = 1 << 5;
        const SD_JOURNAL_INCLUDE_DEFAULT_NAMESPACE = 1 << 6;
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

    pub fn get_logs(&self) -> Result<Vec<HashMap<&str, &str>>, i32> {
        let mut logs = Vec::new();
        
        unsafe {
            let data = CString::new("_PID=1").expect("Could not generate string");
            let ret = sd_journal_add_match(self.ptr, data.as_ptr(), 0usize);

            if ret < 0 {
                return Err(ret);
            }
        }
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
                    },
                    Err(_) => {},
                }
            }

            logs.push(dic);
        }

        Ok(logs)
    }

    pub fn close(&mut self) {
        unsafe {
            sd_journal_close(self.ptr);
        }
    }

    fn get_field(&self, field: &str) -> Result<&str, i32> {
        let mut data: *const c_void = std::ptr::null_mut();
        let mut length: size_t = 0;
        let field = CString::new(field).expect("CString failed");
        let ret: libc::c_int;
        unsafe {
            ret = sd_journal_get_data(self.ptr, field.as_ptr(), &mut data, &mut length);
        }
        if ret < 0 {
            return Err(ret);
        }
        let result = unsafe {
            match CStr::from_ptr(data as *mut c_char).to_str() {
                Ok(s) => Ok(s),
                Err(_) => Err(-1),
            }
        };

        result
    }
}
