use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unit {
    unit_file: String,
    state: String,
    preset: Option<String>
}