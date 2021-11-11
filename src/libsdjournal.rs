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
