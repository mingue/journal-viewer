use libc::{c_char, c_void, size_t};
use std::{ffi::{CStr, CString}, fmt::Display};

mod ffi {
    use libc::{c_char, c_int, c_void, size_t};

    extern "C" {
        // int sd_journal_open(sd_journal **ret, int flags);
        pub fn sd_journal_open(sd_journal: &mut *const c_void, flags: u32) -> c_int;

        //void sd_journal_close(sd_journal *j);
        pub fn sd_journal_close(sd_journal: *const c_void);

        //int sd_journal_next(sd_journal *j);
        pub fn sd_journal_next(sd_journal: *const c_void) -> c_int;

        //int sd_journal_get_data(sd_journal *j, const char *field, const void **data, size_t *length);
        pub fn sd_journal_get_data(
            sd_journal: *const c_void,
            field: *const c_char,
            data: &mut *const c_void,
            size: *mut size_t,
        ) -> c_int;

        //int sd_journal_add_match(sd_journal *j, const void *data, size_t size);
        pub fn sd_journal_add_match(
            sd_journal: *const c_void,
            data: *const c_char,
            size: size_t,
        ) -> c_int;
    }
}

#[derive(Debug)]
pub struct JournalError(i32);

impl Display for JournalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn sd_journal_open(sd_journal: &mut *const c_void, flags: u32) -> Result<(), JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = ffi::sd_journal_open(sd_journal, flags);
    }
    if ret != 0 {
        return Err(JournalError(ret));
    }

    Ok(())
}

pub fn sd_journal_close(sd_journal: *const c_void) {
    unsafe {
        ffi::sd_journal_close(sd_journal);
    }
}

pub fn sd_journal_next(sd_journal: *const c_void) -> Result<bool, JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = ffi::sd_journal_next(sd_journal);
    }

    if ret < 0 {
        return Err(JournalError(ret));
    }

    Ok(ret > 0)
}

pub fn sd_journal_get_data(sd_journal: *const c_void, field: &str) -> Result<String, JournalError> {
    let mut data: *const c_void = std::ptr::null_mut();
    let mut length: size_t = 0;
    let c_field = CString::new(field).expect("CString failed");
    let ret: libc::c_int;
    unsafe {
        ret = ffi::sd_journal_get_data(sd_journal, c_field.as_ptr(), &mut data, &mut length);
    }

    if ret < 0 {
        return Err(JournalError(ret));
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

    if let Err(e) = result {
        return Err(JournalError(e));
    }

    Ok(result.unwrap())
}

pub fn sd_journal_add_match(sd_journal: *const c_void, data: String) -> Result<(), JournalError> {
    let ret: libc::c_int;

    unsafe {
        let data = CString::new(data).expect("Could not set pid");
        ret = ffi::sd_journal_add_match(sd_journal, data.as_ptr(), 0usize);
    }

    if ret < 0 {
        return Err(JournalError(ret));
    }
    
    Ok(())
}
