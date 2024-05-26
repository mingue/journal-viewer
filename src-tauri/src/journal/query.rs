#[derive(Debug)]
pub struct Query {
    pub(crate) pid: u32,
    pub(crate) fields: Vec<String>,
    pub(crate) minimum_priority: u32,
    pub(crate) units: Vec<String>,
    pub(crate) slice: String,
    pub(crate) limit: u64,
    pub(crate) date_less_than: u64,
    pub(crate) date_more_than: u64,
    pub(crate) transports: Vec<String>,
    pub(crate) quick_search: String,
    pub(crate) reset_position: bool,
    pub(crate) boot_ids: Vec<String>,
}
