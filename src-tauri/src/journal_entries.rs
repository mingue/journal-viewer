use std::vec;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JournalEntries {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl JournalEntries {
    pub fn new(lenght: usize) -> JournalEntries {
        JournalEntries{
            headers: vec![],
            rows: Vec::with_capacity(lenght)
        }
    }
}
