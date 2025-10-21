use crate::journal::Boot;
use crate::journal::JournalError;
use crate::journal::Unit;
use crate::journal::{INIT_UNIT, QueryBuilder};
use crate::journal::{Journal, OpenFlags};
use crate::journal::{JournalEntries, JournalEntry};
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use tauri::async_runtime::Mutex;

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
#[instrument]
pub(crate) async fn get_logs(
    mut query: JournalQuery,
    journal: tauri::State<'_, Mutex<Journal>>,
) -> Result<JournalEntries, JournalError> {
    debug!("Getting logs...");

    // If systemd service is specified, remove from unit filter
    // and add it back later as pid filter
    let mut add_init_filter = false;
    if !query.services.is_empty() && query.services[0] == INIT_UNIT {
        query.services.remove(0);
        add_init_filter = true;
    }

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

    // Add back filter for systemd service as pid=1
    if add_init_filter {
        q.with_pid(1);
    }

    let date_from = DateTime::parse_from_rfc3339(&query.datetime_from).ok();
    let date_to = DateTime::parse_from_rfc3339(&query.datetime_to).ok();

    if let Some(x) = date_from {
        q.with_date_more_than(x.timestamp_micros() as u64);
    }

    if let Some(x) = date_to {
        q.with_date_less_than(x.timestamp_micros() as u64);
    } else {
        let datetime_to = Utc::now() + Duration::days(1);
        q.with_date_less_than(datetime_to.timestamp_micros() as u64);
    }

    let q = q.build();

    let lock = journal.lock().await;
    let logs = lock.query_logs(&q)?;
    debug!("Found {} entries.", logs.rows.len());

    Ok(logs)
}

#[tauri::command]
#[instrument]
pub(crate) async fn get_full_entry(timestamp: u64) -> Result<JournalEntry, JournalError> {
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
#[instrument]
pub(crate) async fn get_summary(query: SummaryQuery) -> Result<JournalEntries, JournalError> {
    debug!("Getting summary...");
    let j = Journal::open(
        OpenFlags::SD_JOURNAL_LOCAL_ONLY
            | OpenFlags::SD_JOURNAL_SYSTEM
            | OpenFlags::SD_JOURNAL_CURRENT_USER,
    )
    .unwrap();

    let datetime_from = Utc::now() - Duration::days(5);
    let datetime_to = Utc::now() + Duration::days(1);
    let mut qb = QueryBuilder::default();
    let q = qb
        .with_fields(vec!["__REALTIME".into()])
        .with_limit(10_000)
        .with_date_more_than(datetime_from.timestamp_micros() as u64)
        .with_date_less_than(datetime_to.timestamp_micros() as u64)
        .with_priority_above_or_equal_to(query.priority)
        .build();

    let logs = j.query_logs(&q)?;
    debug!("Found {} entries.", logs.rows.len());

    Ok(logs)
}

#[tauri::command]
#[instrument]
pub(crate) async fn get_services() -> Result<Vec<Unit>, JournalError> {
    debug!("Getting services...");
    let services = Journal::list_services();
    debug!("found {} services", services.len());

    Ok(services)
}

#[tauri::command]
#[instrument]
pub(crate) async fn get_boots() -> Result<Vec<Boot>, JournalError> {
    debug!("Getting boots...");
    let boots = Journal::list_boots();
    debug!("found {} boots", boots.len());

    Ok(boots)
}
