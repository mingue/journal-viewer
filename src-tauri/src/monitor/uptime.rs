use super::SystemStatus;

pub fn read_file(path: &str, ss: &mut SystemStatus) -> anyhow::Result<()> {
    let uptime = std::fs::read_to_string(format!("{path}/uptime"))?;
    let fields: Vec<&str> = uptime.split(' ').collect();
    ss.uptime_seconds = fields[0].parse::<f32>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::monitor::{uptime, SystemStatus};

    #[test]
    fn read_file() -> Result<()>{
        let mut ss = SystemStatus::default();
        uptime::read_file("./tests/fixtures", &mut ss)?;
        assert_eq!(ss.uptime_seconds, 567520.93);

        Ok(())
    }
}
