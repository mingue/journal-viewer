mod cmdline;
mod pid_stat;
mod smaps_rollup;
mod stat;
mod uptime;
// mod meminfo
mod fd;
mod process_status;
mod system_status;

use anyhow::Result;
pub use process_status::ProcessStatus;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashMap, fs::read_dir, process::Command};
pub use system_status::SystemStatus;

lazy_static! {
    static ref CLICKS: usize = get_clicks();
}

fn get_clicks() -> usize {
    let clicks = Command::new("getconf").arg("CLK_TCK").output().unwrap();
    let clicks = String::from_utf8(clicks.stdout).unwrap();
    let clicks = clicks.trim();

    clicks.parse::<usize>().unwrap()
}

#[derive(Default, Debug)]
pub struct Monitor {
    procs_path: &'static str,
    last_processes: Option<HashMap<usize, ProcessStatus>>,
}

impl Monitor {
    pub fn new() -> Monitor {
        Monitor {
            procs_path: "/proc",
            last_processes: None,
        }
    }

    pub fn get_system_status(&self) -> Result<SystemStatus> {
        let mut ss = SystemStatus::default();

        uptime::read_file(self.procs_path, &mut ss)?;
        Ok(ss)
    }

    fn get_running_pids(&self) -> Result<Vec<usize>> {
        let pids: Vec<usize> = read_dir(self.procs_path)?
            .filter_map(|d| {
                match d { Err(e) => {
                    debug!("couldn't read pid info {}", e);
                    None
                } _ => {
                    d.ok()
                }}
            })
            .filter(|d| d.file_type().unwrap().is_dir())
            .filter_map(|d| d.file_name().into_string().ok())
            .filter_map(|d| d.parse::<usize>().ok())
            .collect();

        Ok(pids)
    }

    pub fn get_processes(&mut self) -> Option<&HashMap<usize, ProcessStatus>> {
        debug!("processes started");

        let pids = self.get_running_pids().ok()?;
        debug!("processes found {} pids", pids.len());

        let mut process_entries: HashMap<usize, ProcessStatus> = pids
            .into_par_iter()
            .filter_map(|pid| {
                let pe = self.create_process_entry(&pid);
                match pe {
                    Ok(pe) => Some((pid, pe)),
                    Err(e) => {
                        debug!("couldn't get process info {} ", e);
                        None
                    }
                }
            })
            .collect();
        debug!("processes found {} process entries", process_entries.len());

        process_entries.iter_mut().for_each(|(k, v)| {
            v.time_userspace_miliseconds =
                (v.time_userspace_clicks as f32 / *CLICKS as f32) * 1000f32;
            v.time_kernel_miliseconds = (v.time_kernel_clicks as f32 / *CLICKS as f32) * 1000f32;

            if let Some(last_processes) = &self.last_processes {
                match last_processes.get(k) {
                    Some(last) => {
                        let elapsed_miliseconds =
                            (v.scrapped_timestamp - last.scrapped_timestamp).num_milliseconds();
                        let current_cpu_usage_miliseconds =
                            v.time_userspace_miliseconds + v.time_kernel_miliseconds;
                        let last_cpu_usage_miliseconds =
                            last.time_userspace_miliseconds + last.time_kernel_miliseconds;
                        let cpu_usage_miliseconds =
                            current_cpu_usage_miliseconds - last_cpu_usage_miliseconds;
                        v.cpu_usage_percentage =
                            cpu_usage_miliseconds * 100f32 / elapsed_miliseconds as f32;
                    }
                    None => debug!("New process"),
                }
            }
        });

        trace!("processes {:?}", &process_entries);

        // Not supported by borrow checker, so have to use get_or_insert to trick it
        // Ok(&self.last_processes.unwrap())

        self.last_processes = Some(process_entries);

        match &self.last_processes {
            Some(lp) => {
                debug!("processes {:?}", lp.len());
                Some(lp)
            }
            None => {
                debug!("processes none found");
                None
            }
        }
    }

    fn create_process_entry(&self, pid: &usize) -> Result<ProcessStatus> {
        let mut pe = ProcessStatus {
            pid: *pid,
            ..ProcessStatus::default()
        };

        cmdline::read_file(self.procs_path, pid, &mut pe)?;
        pid_stat::read_file(self.procs_path, pid, &mut pe)?;
        smaps_rollup::read_file(self.procs_path, pid, &mut pe)?;
        fd::read_file(self.procs_path, pid, &mut pe)?;

        Ok(pe)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::Duration;

    use crate::monitor::{Monitor, ProcessStatus, CLICKS};

    #[test]
    fn get_running_pids() -> Result<()> {
        let monitor = Monitor {
            procs_path: "./tests/fixtures/",
            last_processes: None,
        };
        let pids = monitor.get_running_pids()?;
        assert!(!pids.is_empty());

        Ok(())
    }

    #[test]
    fn get_process_entries() -> Result<()> {
        let mut monitor = Monitor {
            procs_path: "./tests/fixtures/",
            last_processes: None,
        };
        let processes = monitor.get_processes().unwrap();
        assert!(!processes.is_empty());

        Ok(())
    }

    #[test]
    fn get_process_entries_with_cpu_percentage() -> Result<()> {
        let mut monitor = Monitor {
            procs_path: "./tests/fixtures/",
            last_processes: None,
        };
        let processes = monitor.get_processes().unwrap();
        unsafe {
            // Hack to modify state of existing process read
            let p1 = processes.get(&1).unwrap() as *const ProcessStatus;
            let p1 = p1 as *mut ProcessStatus;
            (*p1).scrapped_timestamp = chrono::Utc::now() - Duration::seconds(5);
            (*p1).time_userspace_miliseconds -= 4000f32;
        }

        let processes = monitor.get_processes().unwrap();
        let p1 = processes.get(&1).unwrap();
        assert!(p1.cpu_usage_percentage >= 80f32);

        Ok(())
    }

    #[test]
    fn system_status() -> Result<()> {
        let monitor = Monitor {
            procs_path: "./tests/fixtures/",
            last_processes: None,
        };
        let _ss = monitor.get_system_status()?;

        Ok(())
    }

    #[test]
    fn clicks() {
        assert_eq!(*CLICKS, 100);
    }
}
