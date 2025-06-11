use nom::{bytes::complete::*, character::complete::*};

use super::ProcessStatus;

pub fn read_file(
    procs_path: &str,
    pid: &usize,
    process_entry: &mut ProcessStatus,
) -> anyhow::Result<()> {
    if let Ok(smaps_rollup) = std::fs::read_to_string(format!("{procs_path}/{pid}/smaps_rollup")) {
        let lines = smaps_rollup.lines();

        let lines: Vec<String> = lines.skip(1).map(|l| l.to_string()).collect();

        let fields: Vec<SmapLine> = lines
            .iter()
            .map(|l| parse_line(l))
            .filter_map(|r| r.ok())
            .map(|r| r.1)
            .collect();

        process_entry.pss_in_kb = fields.iter().find(|sl| sl.label == "Pss").unwrap().kb;
        process_entry.rss_in_kb = fields.iter().find(|sl| sl.label == "Rss").unwrap().kb;
        process_entry.uss_in_kb =
            process_entry.pss_in_kb - fields.iter().find(|sl| sl.label == "Pss_Shmem").unwrap().kb;
    }

    Ok(())
}

fn parse_line(l: &str) -> nom::IResult<&str, SmapLine> {
    let (r, label) = take_until(":")(l)?;
    let (r, _) = tag(":")(r)?;
    let (r, _) = multispace0(r)?;
    let (r, kb) = digit0(r)?;
    let kb = kb.parse::<usize>().expect("error parsing kb");
    let (r, _) = multispace0(r)?;
    let (_, _) = tag("kB")(r)?;

    Ok((l, SmapLine { label, kb }))
}

struct SmapLine<'a> {
    label: &'a str,
    kb: usize,
}

#[cfg(test)]
mod tests {
    use crate::monitor::{ProcessStatus, smaps_rollup};
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
