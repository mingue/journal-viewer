use crate::libsdjournal_bindings;
use libc::{c_char, c_void, size_t};
use serde::Serialize;
use std::{
    ffi::{CStr, CString},
    fmt::Display,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalError(pub i32);

impl Display for JournalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn sd_journal_open(sd_journal: &mut *mut c_void, flags: u32) -> Result<(), JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = libsdjournal_bindings::sd_journal_open(sd_journal, flags);
    }
    if ret != 0 {
        return Err(JournalError(ret));
    }

    Ok(())
}

pub fn sd_journal_close(sd_journal: *mut c_void) {
    unsafe {
        libsdjournal_bindings::sd_journal_close(sd_journal);
    }
}

pub fn sd_journal_next(sd_journal: *mut c_void) -> Result<bool, JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = libsdjournal_bindings::sd_journal_next(sd_journal);
    }

    if ret < 0 {
        return Err(JournalError(ret));
    }

    Ok(ret > 0)
}

pub fn sd_journal_previous(sd_journal: *mut c_void) -> Result<bool, JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = libsdjournal_bindings::sd_journal_previous(sd_journal);
    }

    if ret < 0 {
        return Err(JournalError(ret));
    }

    Ok(ret > 0)
}

pub fn sd_journal_next_skip(sd_journal: *mut c_void, skip: u64) -> Result<bool, JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = libsdjournal_bindings::sd_journal_next_skip(sd_journal, skip);
    }

    if ret < 0 {
        return Err(JournalError(ret));
    }

    Ok(ret > 0)
}

pub fn sd_journal_previous_skip(sd_journal: *mut c_void, skip: u64) -> Result<bool, JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = libsdjournal_bindings::sd_journal_previous_skip(sd_journal, skip);
    }

    if ret < 0 {
        return Err(JournalError(ret));
    }

    Ok(ret > 0)
}

pub fn sd_journal_get_data(sd_journal: *mut c_void, field: &str) -> Result<String, JournalError> {
    let mut data: *mut c_void = std::ptr::null_mut();
    let mut length: size_t = 0;
    let c_field = CString::new(field).expect("CString failed");
    let ret: libc::c_int;
    unsafe {
        ret = libsdjournal_bindings::sd_journal_get_data(
            sd_journal,
            c_field.as_ptr(),
            &mut data,
            &mut length,
        );
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

pub fn sd_journal_add_match(sd_journal: *mut c_void, data: String) -> Result<(), JournalError> {
    let ret: libc::c_int;

    unsafe {
        let data = CString::new(data).expect("Could not set pid");
        ret = libsdjournal_bindings::sd_journal_add_match(sd_journal, data.as_ptr(), 0usize);
    }

    if ret < 0 {
        return Err(JournalError(ret));
    }

    Ok(())
}

pub fn sd_journal_seek_head(sd_journal: *mut c_void) -> Result<(), JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = libsdjournal_bindings::sd_journal_seek_head(sd_journal);
    }
    if ret != 0 {
        return Err(JournalError(ret));
    }

    Ok(())
}

pub fn sd_journal_seek_tail(sd_journal: *mut c_void) -> Result<(), JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = libsdjournal_bindings::sd_journal_seek_tail(sd_journal);
    }
    if ret != 0 {
        return Err(JournalError(ret));
    }

    Ok(())
}

pub fn sd_journal_add_conjunction(sd_journal: *mut c_void) -> Result<(), JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = libsdjournal_bindings::sd_journal_add_conjunction(sd_journal);
    }

    if ret < 0 {
        return Err(JournalError(ret));
    }

    Ok(())
}

pub fn sd_journal_add_disjunction(sd_journal: *mut c_void) -> Result<(), JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = libsdjournal_bindings::sd_journal_add_disjunction(sd_journal);
    }

    if ret < 0 {
        return Err(JournalError(ret));
    }

    Ok(())
}

pub fn sd_journal_flush_matches(sd_journal: *mut c_void) {
    unsafe {
        libsdjournal_bindings::sd_journal_flush_matches(sd_journal);
    }
}

pub fn sd_journal_get_realtime_usec(
    sd_journal: *mut c_void,
    microseconds: &mut u64,
) -> Result<(), JournalError> {
    let ret: libc::c_int;

    unsafe {
        ret = libsdjournal_bindings::sd_journal_get_realtime_usec(sd_journal, microseconds);
    }

    if ret < 0 {
        return Err(JournalError(ret));
    }

    Ok(())
}
