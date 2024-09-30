use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProcessStatus {
    pub pid: usize,
    pub cmd: String,
    pub process_name: String,
    pub pss_in_kb: usize,
    pub rss_in_kb: usize,
    pub uss_in_kb: usize, // Pss - Pss_Shmem
    pub(super)time_userspace_clicks: usize,
    pub(super)time_kernel_clicks: usize,
    pub time_userspace_miliseconds: f32,
    pub time_kernel_miliseconds: f32,
    pub start_time: usize,
    pub cpu_usage_percentage: f32,
    pub(super) scrapped_timestamp: DateTime<Utc>,
}
