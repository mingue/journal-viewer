#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod journal;
mod journal_entries;
mod libsdjournal;
mod query_builder;

#[macro_use]
extern crate log;

use crate::query_builder::{fields, QueryBuilder};
use journal::{Journal, OpenFlags};
use journal_entries::JournalEntries;
use serde::{Deserialize, Serialize};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_logs, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Deserialize)]
pub struct JournalQuery {
    fields: Vec<String>,
    priority: u32,
    offset: u64,
    limit: u64,
}

#[tauri::command]
fn get_logs(query: JournalQuery) -> JournalEntries {
    let j = Journal::open(
        OpenFlags::SD_JOURNAL_LOCAL_ONLY
            | OpenFlags::SD_JOURNAL_SYSTEM
            | OpenFlags::SD_JOURNAL_CURRENT_USER,
    )
    .unwrap();

    let mut qb = QueryBuilder::default();
    //let jQuery: JournalQuery = serde_json::from_str(&query).unwrap();
    qb.with_fields(query.fields)
        .with_offset(query.offset)
        .with_limit(query.limit)
        .with_priority_above(query.priority)
        .unwrap();
    let logs = j.query_logs(&qb).unwrap();

    logs
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
