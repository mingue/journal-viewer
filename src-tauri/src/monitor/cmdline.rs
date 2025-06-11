use super::ProcessStatus;

pub fn read_file(path: &str, pid: &usize, pe: &mut ProcessStatus) -> anyhow::Result<()> {
    if let Ok(mut cmd_line) = std::fs::read_to_string(format!("{path}/{pid}/cmdline")) {
        cmd_line = cmd_line.trim_matches(char::from(0)).to_owned();
        pe.cmd = cmd_line;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::monitor::{ProcessStatus, cmdline};
    use anyhow::Result;

    #[test]
    fn read_file() -> Result<()> {
        let mut pe = ProcessStatus::default();
        cmdline::read_file("./tests/fixtures", &1, &mut pe)?;
        assert_eq!(pe.cmd, "/sbin/init");

        Ok(())
    }
}
