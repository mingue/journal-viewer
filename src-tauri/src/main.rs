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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_logs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_logs() -> String {
    let j = Journal::open(
        OpenFlags::SD_JOURNAL_LOCAL_ONLY
            | OpenFlags::SD_JOURNAL_SYSTEM
            | OpenFlags::SD_JOURNAL_CURRENT_USER,
    )
    .unwrap();

    let mut qb = QueryBuilder::default();
    qb.with_fields(vec![fields::SOURCE_REALTIME_TIMESTAMP, fields::MESSAGE]);
    //qb.with_pid(1);

    let logs = j.query_logs(&qb).unwrap();

    serde_json::to_string(&logs).unwrap()
}
