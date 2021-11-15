mod journal;
mod libsdjournal;
mod query_builder;

use journal::{Journal, OpenFlags};
use std::collections::HashMap;

use crate::query_builder::QueryBuilder;

fn main() {
    let mut j = Journal::open(
        OpenFlags::SD_JOURNAL_SYSTEM
            | OpenFlags::SD_JOURNAL_CURRENT_USER
            | OpenFlags::SD_JOURNAL_LOCAL_ONLY,
    )
    .expect("Could not open the Journal");

    let mut qb = QueryBuilder::default();
    qb.with_pid(1);

    match j.query_logs(&qb) {
        Ok(logs) => {
            print_logs(logs);
        }
        Err(..) => {}
    }

    j.close();
    println!("{:?}", j);
}

fn print_logs(logs: Vec<HashMap<&str, String>>) {
    for log in logs {
        for field in log.iter() {
            print!("{}: {} ", field.0, field.1);
        }
        println!("");
    }
}
