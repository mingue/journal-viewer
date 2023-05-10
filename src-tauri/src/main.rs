#![allow(dead_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod boot;
mod journal;
mod journal_entries;
mod journal_fields;
mod libsdjournal;
mod libsdjournal_bindings;
mod query;
mod query_builder;
mod unit;

#[macro_use]
extern crate log;

use crate::query_builder::QueryBuilder;
use boot::Boot;
use chrono::{DateTime, Duration, Utc};
use env_logger::Env;
use journal::{Journal, OpenFlags};
use journal_entries::{JournalEntries, JournalEntry};
use libsdjournal::JournalError;
use serde::Deserialize;
use tauri::async_runtime::Mutex;
use unit::Unit;

fn main() {
    let env = Env::default()
        .filter_or("RUST_LOG", "warn")
        .write_style_or("RUST_LOG_STYLE", "never");

    env_logger::init_from_env(env);

    let j = Journal::open(
        OpenFlags::SD_JOURNAL_LOCAL_ONLY
            | OpenFlags::SD_JOURNAL_SYSTEM
            | OpenFlags::SD_JOURNAL_CURRENT_USER,
    )
    .unwrap();

    info!("Starting journal logger");
    tauri::Builder::default()
        .manage(Mutex::new(j))
        .invoke_handler(tauri::generate_handler![
            get_logs,
            get_summary,
            get_services,
            get_full_entry,
            get_boots,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalQuery {
    fields: Vec<String>,
    priority: u32,
    limit: u64,
    quick_search: String,
    reset_position: bool,
    services: Vec<String>,
    transports: Vec<String>,
    datetime_from: String,
    datetime_to: String,
    boot_ids: Vec<String>,
}

#[tauri::command]
async fn get_logs(
    query: JournalQuery,
    journal: tauri::State<'_, Mutex<Journal>>,
) -> Result<JournalEntries, JournalError> {
    debug!("Getting logs...");

    let mut qb = QueryBuilder::default();
    let q = qb
        .with_fields(query.fields)
        .with_limit(query.limit)
        .with_quick_search(query.quick_search)
        .reset_position(query.reset_position)
        .with_priority_above_or_equal_to(query.priority)
        .with_units(query.services)
        .with_transports(query.transports)
        .with_boot_ids(query.boot_ids);

    let date_from = DateTime::parse_from_rfc3339(&query.datetime_from).ok();
    let date_to = DateTime::parse_from_rfc3339(&query.datetime_to).ok();

    if let Some(x) = date_from {
        q.with_date_not_more_recent_than(x.timestamp_micros() as u64);
    }

    if let Some(x) = date_to {
        q.with_date_not_older_than(x.timestamp_micros() as u64);
    }

    let q = q.build();

    let lock = journal.lock().await;
    let logs = lock.query_logs(&q)?;
    debug!("Found {} entries.", logs.rows.len());

    Ok(logs)
}

#[tauri::command]
async fn get_full_entry(timestamp: u64) -> Result<JournalEntry, JournalError> {
    debug!("Getting full entry for timestamp {}...", timestamp);

    let j = Journal::open(
        OpenFlags::SD_JOURNAL_LOCAL_ONLY
            | OpenFlags::SD_JOURNAL_SYSTEM
            | OpenFlags::SD_JOURNAL_CURRENT_USER,
    )
    .unwrap();

    let entry = j.get_full_entry(timestamp)?;

    debug!("Found entry for timestamp {}", timestamp);

    Ok(entry)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryQuery {
    priority: u32,
}

#[tauri::command]
async fn get_summary(query: SummaryQuery) -> Result<JournalEntries, JournalError> {
    debug!("Getting summary...");
    let j = Journal::open(
        OpenFlags::SD_JOURNAL_LOCAL_ONLY
            | OpenFlags::SD_JOURNAL_SYSTEM
            | OpenFlags::SD_JOURNAL_CURRENT_USER,
    )
    .unwrap();

    let datetime_to = Utc::now() - Duration::days(1);
    let mut qb = QueryBuilder::default();
    let q = qb
        .with_fields(vec!["__REALTIME".into()])
        .with_limit(10_000)
        .with_date_not_older_than(datetime_to.timestamp_micros() as u64)
        .with_priority_above_or_equal_to(query.priority)
        .build();

    let logs = j.query_logs(&q)?;
    debug!("Found {} entries.", logs.rows.len());

    Ok(logs)
}

#[tauri::command]
async fn get_services() -> Result<Vec<Unit>, JournalError> {
    debug!("Getting services...");
    let services = Journal::list_services();
    debug!("found {} services", services.len());

    Ok(services)
}

#[tauri::command]
async fn get_boots() -> Result<Vec<Boot>, JournalError> {
    debug!("Getting boots...");
    let boots = Journal::list_boots();
    debug!("found {} boots", boots.len());

    Ok(boots)
}
