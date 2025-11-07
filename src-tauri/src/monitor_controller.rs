use crate::journal::JournalError;
use crate::monitor::Monitor;
use crate::monitor::ProcessStatus;
use crate::monitor::SystemStatus;
use serde::Deserialize;
use tauri::async_runtime::Mutex;

#[tauri::command]
#[instrument]
pub(crate) async fn get_system_status(
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
#[instrument]
pub(crate) async fn get_processes(
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
