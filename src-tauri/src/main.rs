#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod journal;
mod journal_entries;
mod journal_fields;
mod libsdjournal;
mod libsdjournal_bindings;
mod query_builder;

#[macro_use]
extern crate log;

use crate::query_builder::QueryBuilder;
use chrono::{Duration, Utc};
use env_logger::Env;
use journal::{Journal, OpenFlags};
use journal_entries::JournalEntries;
use serde::Deserialize;

fn main() {
    let env = Env::default()
        .filter_or("RUST_LOG", "trace")
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    info!("Starting journal logger");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_logs, greet, get_logs_summary])
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
    debug!("Getting logs...");
    let j = Journal::open(
        OpenFlags::SD_JOURNAL_LOCAL_ONLY
            | OpenFlags::SD_JOURNAL_SYSTEM
            | OpenFlags::SD_JOURNAL_CURRENT_USER,
    )
    .unwrap();

    let mut qb = QueryBuilder::default();
    qb.with_fields(query.fields)
        .with_offset(query.offset)
        .with_limit(query.limit)
        .with_priority_above_or_equal_to(query.priority);

    let logs = j.query_logs(&qb).unwrap();
    debug!("Found {} entries.", logs.rows.len());

    logs
}

#[tauri::command]
fn get_logs_summary(query: JournalQuery) -> JournalEntries {
    debug!("Getting summary...");
    let j = Journal::open(
        OpenFlags::SD_JOURNAL_LOCAL_ONLY
            | OpenFlags::SD_JOURNAL_SYSTEM
            | OpenFlags::SD_JOURNAL_CURRENT_USER,
    )
    .unwrap();

    let from = Utc::now() - Duration::days(2);
    let mut qb = QueryBuilder::default();
    qb.with_fields(vec![journal_fields::SOURCE_REALTIME_TIMESTAMP.into()])
        .with_offset(query.offset)
        .with_limit(20_000)
        .with_date_from(from.timestamp_micros() as u64)
        .with_priority_above_or_equal_to(query.priority);

    let logs = j.query_logs(&qb).unwrap();
    debug!("Found {} entries.", logs.rows.len());

    logs
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
