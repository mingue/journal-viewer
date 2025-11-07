#![allow(clippy::pedantic)]
#![allow(dead_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod journal;
mod journal_controller;
mod monitor;
mod monitor_controller;

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::str::FromStr;

use crate::journal::Journal;
use crate::journal::JournalError;
use crate::journal::OpenFlags;
use crate::monitor::Monitor;
use serde::Deserialize;
use serde::Serialize;
use tauri::async_runtime::Mutex;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

fn main() {
    let fmt_layer = fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

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
            journal_controller::get_logs,
            journal_controller::get_summary,
            journal_controller::get_services,
            journal_controller::get_full_entry,
            journal_controller::get_boots,
            monitor_controller::get_system_status,
            monitor_controller::get_processes,
            get_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    system_monitor_enabled: bool,
}

#[tauri::command]
#[instrument]
fn get_config() -> Result<Config, JournalError> {
    let mut config = Config {
        system_monitor_enabled: false,
    };

    if let Ok(monitor_enabled) = env::var("JV_MONITOR_ENABLED") {
        config.system_monitor_enabled = FromStr::from_str(&monitor_enabled).unwrap();
    }
    Ok(config)
}
