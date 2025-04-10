#![allow(dead_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod journal;
mod monitor;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use chrono::{DateTime, Duration, Utc};
use env_logger::Env;
use journal::Boot;
use journal::JournalError;
use journal::Unit;
use journal::{INIT_UNIT, QueryBuilder};
use journal::{Journal, OpenFlags};
use journal::{JournalEntries, JournalEntry};
use monitor::Monitor;
use monitor::ProcessStatus;
use monitor::SystemStatus;
use serde::Deserialize;
use tauri::async_runtime::Mutex;

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

    let m = Monitor::new();

    info!("Starting journal logger");
    tauri::Builder::default()
        .manage(Mutex::new(j))
        .manage(Mutex::new(m))
        .invoke_handler(tauri::generate_handler![
            get_logs,
            get_summary,
            get_services,
            get_full_entry,
            get_boots,
            get_system_status,
            get_processes
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

#[tauri::command]
async fn get_system_status(
    monitor: tauri::State<'_, Mutex<Monitor>>,
) -> Result<SystemStatus, JournalError> {
    debug!("Getting system status...");
    let m = monitor.lock().await;

    match m.get_system_status() {
        Ok(ss) => {
            debug!("Got system status...");
            Ok(ss)
        }
        Err(e) => {
            error!("{:?}", e);
            Err(JournalError::Internal(1))
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessQuery {
    sort_by: String,
    sort_order: String,
}

#[tauri::command]
async fn get_processes(
    query: ProcessQuery,
    monitor: tauri::State<'_, Mutex<Monitor>>,
) -> Result<Vec<ProcessStatus>, JournalError> {
    debug!("Getting processes...");
    let mut m = monitor.lock().await;

    match m.get_processes() {
        Some(p) => {
            debug!("Got {} processes", p.len());
            let mut processes: Vec<ProcessStatus> = p.clone().into_values().collect();

            sort_processes(&mut processes, &query);
            Ok(processes[..30].to_vec())
        }
        None => {
            debug!("No processes");
            Err(JournalError::Internal(1))
        }
    }
}

fn sort_processes(processes: &mut [ProcessStatus], query: &ProcessQuery) {
    match query.sort_by.as_str() {
        "pid" => {
            processes.sort_by(|a, b| {
                if query.sort_order.to_lowercase() == "desc" {
                    b.pid.cmp(&a.pid)
                } else {
                    a.pid.cmp(&b.pid)
                }
            });
        }
        "cpu_usage_percentage" => {
            processes.sort_by(|a, b| {
                if query.sort_order.to_lowercase() == "desc" {
                    b.cpu_usage_percentage.total_cmp(&a.cpu_usage_percentage)
                } else {
                    a.cpu_usage_percentage.total_cmp(&b.cpu_usage_percentage)
                }
            });
        }
        "rss_in_kb" => {
            processes.sort_by(|a, b| {
                if query.sort_order.to_lowercase() == "desc" {
                    b.rss_in_kb.cmp(&a.rss_in_kb)
                } else {
                    a.rss_in_kb.cmp(&b.rss_in_kb)
                }
            });
        }
        "uss_in_kb" => {
            processes.sort_by(|a, b| {
                if query.sort_order.to_lowercase() == "desc" {
                    b.uss_in_kb.cmp(&a.uss_in_kb)
                } else {
                    a.uss_in_kb.cmp(&b.uss_in_kb)
                }
            });
        }
        "pss_in_kb" => {
            processes.sort_by(|a, b| {
                if query.sort_order.to_lowercase() == "desc" {
                    b.pss_in_kb.cmp(&a.pss_in_kb)
                } else {
                    a.pss_in_kb.cmp(&b.pss_in_kb)
                }
            });
        }
        "process_name" => {
            processes.sort_by(|a, b| {
                if query.sort_order.to_lowercase() == "desc" {
                    b.process_name.cmp(&a.process_name)
                } else {
                    a.process_name.cmp(&b.process_name)
                }
            });
        }
        "time_userspace_miliseconds" => {
            processes.sort_by(|a, b| {
                if query.sort_order.to_lowercase() == "desc" {
                    b.time_userspace_miliseconds
                        .total_cmp(&a.time_userspace_miliseconds)
                } else {
                    a.time_userspace_miliseconds
                        .total_cmp(&b.time_userspace_miliseconds)
                }
            });
        }
        "time_kernel_miliseconds" => {
            processes.sort_by(|a, b| {
                if query.sort_order.to_lowercase() == "desc" {
                    b.time_kernel_miliseconds
                        .total_cmp(&a.time_kernel_miliseconds)
                } else {
                    a.time_kernel_miliseconds
                        .total_cmp(&b.time_kernel_miliseconds)
                }
            });
        }
        "cmd" => {
            processes.sort_by(|a, b| {
                if query.sort_order.to_lowercase() == "desc" {
                    b.cmd.cmp(&a.cmd)
                } else {
                    a.cmd.cmp(&b.cmd)
                }
            });
        }
        "fds" => {
            processes.sort_by(|a, b| {
                if query.sort_order.to_lowercase() == "desc" {
                    b.fds.cmp(&a.fds)
                } else {
                    a.fds.cmp(&b.fds)
                }
            });
        }
        default => {
            info!("Unknown sort_by field: {}", default);
        }
    }
}
