use super::ProcessStatus;

pub fn read_file(
    procs_path: &str,
    pid: &usize,
    process_entry: &mut ProcessStatus,
) -> anyhow::Result<()> {
    let stat = std::fs::read_to_string(format!("{procs_path}/{pid}/stat"))?;
    let fields: Vec<&str> = stat.split(' ').collect();

    // Get process name and remove parentesis
    process_entry.process_name = fields[1][1..fields[1].len() - 1].to_owned();
    // Get userspace time in clicks
    // Get kernel/system time in clicks
    process_entry.time_userspace_clicks = fields[13].parse::<usize>()?;
    process_entry.time_kernel_clicks = fields[14].parse::<usize>()?;
    process_entry.start_time = fields[21].parse::<usize>()?;
    process_entry.scrapped_timestamp = chrono::Utc::now();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::monitor::{stat, ProcessStatus};
    use anyhow::Result;

    #[test]
    fn read_file() -> Result<()> {
        let mut pe = ProcessStatus::default();
        stat::read_file("./tests/fixtures", &1, &mut pe)?;

        assert_eq!(pe.process_name, "systemd");
        assert_eq!(pe.time_userspace_clicks, 7278);
        assert_eq!(pe.time_kernel_clicks, 4048);
        assert_eq!(pe.start_time, 12);

        Ok(())
    }
}
