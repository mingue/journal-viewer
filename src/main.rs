mod journal;
mod libsdjournal;
mod query_builder;

#[macro_use]
extern crate log;

use journal::{Journal, OpenFlags};
use libsdjournal::JournalError;
use std::collections::HashMap;

use crate::query_builder::QueryBuilder;

fn main() -> Result<(), JournalError> {
    let j = Journal::open(
        OpenFlags::SD_JOURNAL_SYSTEM
            | OpenFlags::SD_JOURNAL_CURRENT_USER
            | OpenFlags::SD_JOURNAL_LOCAL_ONLY,
    )?;

    let mut qb = QueryBuilder::default();
    qb.with_pid(1);

    let logs = j.query_logs(&qb)?;
    print_logs(logs);
    println!("{:?}", j);

    Ok(())
}

fn print_logs(logs: Vec<HashMap<&str, String>>) {
    for log in logs {
        for field in log.iter() {
            print!("{}: {} ", field.0, field.1);
        }
        println!("");
    }
}
