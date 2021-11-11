mod journal;
mod libsdjournal;
mod query_builder;

use journal::{Journal, OpenFlags};
use std::collections::HashMap;

fn main() {
    let mut j = Journal::open(OpenFlags::SD_JOURNAL_SYSTEM | OpenFlags::SD_JOURNAL_CURRENT_USER)
        .expect("Could not open the Journal");

    match j.get_logs() {
        Ok(logs) => {
            print_logs(logs);
        }
        Err(..) => {}
    }

    j.close();
    println!("{:?}", j);
}

fn print_logs(logs: Vec<HashMap<&str, &str>>) {
    for log in logs {
        if log.contains_key("_COMM") && log.contains_key("MESSAGE") {
            println!("{}: {}", log["_COMM"], log["MESSAGE"]);
        } else if log.contains_key("MESSAGE") {
            println!("{}", log["MESSAGE"]);
        } else {
            println!("Couldn't read the entry");
        }
    }
}
