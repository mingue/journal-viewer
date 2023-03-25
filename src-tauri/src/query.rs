#[derive(Debug)]
pub struct Query {
    pub(crate) pid: u32,
    pub(crate) fields: Vec<String>,
    pub(crate) minimum_priority: u32,
    pub(crate) unit: String,
    pub(crate) slice: String,
    pub(crate) boot_id: String,
    pub(crate) limit: u64,
    pub(crate) from_epoch: u64,
    pub(crate) transports: Vec<String>,
    pub(crate) quick_search: String,
    pub(crate) reset_position: bool,
}