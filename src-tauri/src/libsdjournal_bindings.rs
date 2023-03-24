use libc::{c_char, c_int, c_ulong, c_void, size_t};

extern "C" {
    // int sd_journal_open(sd_journal **ret, int flags);
    pub fn sd_journal_open(sd_journal: &mut *mut c_void, flags: u32) -> c_int;

    //void sd_journal_close(sd_journal *j);
    pub fn sd_journal_close(sd_journal: *mut c_void);

    //int sd_journal_next(sd_journal *j);
    pub fn sd_journal_next(sd_journal: *mut c_void) -> c_int;

    //int sd_journal_previous(sd_journal *j);
    pub fn sd_journal_previous(sd_journal: *mut c_void) -> c_int;

    //int sd_journal_get_data(sd_journal *j, const char *field, const void **data, size_t *length);
    pub fn sd_journal_get_data(
        sd_journal: *mut c_void,
        field: *const c_char,
        data: &mut *mut c_void,
        size: *mut size_t,
    ) -> c_int;

    //int sd_journal_add_match(sd_journal *j, const void *data, size_t size);
    pub fn sd_journal_add_match(
        sd_journal: *mut c_void,
        data: *const c_char,
        size: size_t,
    ) -> c_int;

    //int sd_journal_seek_head(sd_journal *j);
    pub fn sd_journal_seek_head(sd_journal: *mut c_void) -> c_int;

    //int sd_journal_seek_tail(sd_journal *j);
    pub fn sd_journal_seek_tail(sd_journal: *mut c_void) -> c_int;

    //int sd_journal_next_skip(sd_journal *j, uint64_t skip);
    pub fn sd_journal_next_skip(sd_journal: *mut c_void, skip: c_ulong) -> c_int;

    //int sd_journal_previous_skip(sd_journal *j, uint64_t skip);
    pub fn sd_journal_previous_skip(sd_journal: *mut c_void, skip: u64) -> c_int;

    //int sd_journal_add_disjunction(sd_journal *j);
    pub fn sd_journal_add_disjunction(sd_journal: *mut c_void) -> c_int;

    //int sd_journal_add_conjunction(sd_journal *j);
    pub fn sd_journal_add_conjunction(sd_journal: *mut c_void) -> c_int;

    //void sd_journal_flush_matches(sd_journal *j);
    pub fn sd_journal_flush_matches(sd_journal: *mut c_void);
}
