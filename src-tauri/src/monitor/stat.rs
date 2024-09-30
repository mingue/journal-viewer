use nom::{
    Needed, bytes::complete::*,
};

use super::SystemStatus;

pub fn read_file(procs_path: &str, system_status: &mut SystemStatus) -> anyhow::Result<()> {
    let stat = std::fs::read_to_string(format!("{procs_path}/stat"))?;
    let lines = stat.lines();

    let lines: Vec<String> = lines.map(|l| l.to_string()).collect();

    let fields: Vec<StatLine> = lines
        .iter()
        .filter(|l| l.starts_with("cpu"))
        .map(|l| parse_line(l))
        .filter_map(|r| r.ok())
        .map(|r| r.1)
        .collect();

    system_status.idle_time_clicks = fields[0].idle_time_clicks;
    system_status.kernel_mode_clicks = fields[0].kernel_mode_clicks;
    system_status.user_mode_clicks = fields[0].user_mode_clicks;
    // TODO: Add per cpu usage

    Ok(())
}

fn parse_line(l: &str) -> nom::IResult<&str, StatLine> {
    let (r, _) = tag("cpu")(l)?;
    let (r, core_nr) = take_until(" ")(r)?;
    let core_nr = core_nr
        .parse::<usize>()
        .map_err(|_e| nom::Err::Incomplete(Needed::Unknown))?;
    let (r, _) = take_while(|c| c == ' ')(r)?;
    let fields: Vec<&str> = r.split(' ').collect();
    let normal = fields[0]
        .parse::<usize>()
        .map_err(|_e| nom::Err::Incomplete(Needed::Unknown))?;
    let kernel = fields[2]
        .parse::<usize>()
        .map_err(|_e| nom::Err::Incomplete(Needed::Unknown))?;
    let idle = fields[3]
        .parse::<usize>()
        .map_err(|_e| nom::Err::Incomplete(Needed::Unknown))?;

    Ok((
        l,
        StatLine {
            core_nr,
            user_mode_clicks: normal,
            kernel_mode_clicks: kernel,
            idle_time_clicks: idle,
        },
    ))
}

struct StatLine {
    core_nr: usize,
    user_mode_clicks: usize,
    kernel_mode_clicks: usize,
    idle_time_clicks: usize,
}

#[cfg(test)]
mod tests {
    use crate::monitor::{smaps_rollup, ProcessStatus};
    use anyhow::Result;

    #[test]
    fn read_file() -> Result<()> {
        let mut pe: ProcessStatus = ProcessStatus::default();
        smaps_rollup::read_file("./tests/fixtures", &1, &mut pe)?;
        assert_eq!(pe.pss_in_kb, 50469);
        assert_eq!(pe.rss_in_kb, 86828);
        assert_eq!(pe.uss_in_kb, 50469 - 7056);
        Ok(())
    }
}
