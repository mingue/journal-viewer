use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unit {
    pub unit_file: String,
    pub state: String,
    pub preset: Option<String>
}