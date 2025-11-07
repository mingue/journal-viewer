use std::os::unix::prelude::MetadataExt;

use super::ProcessStatus;

pub fn read_file(
    procs_path: &str,
    pid: &usize,
    process_entry: &mut ProcessStatus,
) -> anyhow::Result<()> {
    // Obtain size from stats of fd for fast access to open fds
    if let Ok(stat) = std::fs::metadata(format!("{procs_path}/{pid}/fd")) {
        process_entry.fds = stat.size();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::monitor::{ProcessStatus, fd};
    use anyhow::Result;

    #[test]
    fn read_file() -> Result<()> {
        let mut pe = ProcessStatus::default();
        fd::read_file("./tests/fixtures", &1, &mut pe)?;

        assert_eq!(pe.fds, 2);
        Ok(())
    }
}
