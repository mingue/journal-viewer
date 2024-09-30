use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SystemStatus {
    pub uptime_seconds: f32,
    pub user_mode_clicks: usize,
    pub kernel_mode_clicks: usize,
    pub idle_time_clicks: usize,
}