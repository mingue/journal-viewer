use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Boot {
    index: i32,
    boot_id: String,
    first_entry: i64,
    last_entry: i64,
}
