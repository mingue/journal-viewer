use std::mem;

use super::{journal_fields, query::Query};

pub struct QueryBuilder {
    query: Query,
}

impl QueryBuilder {
    pub fn default() -> Self {
        let query = Query {
            pid: 0,
            fields: vec![],
            minimum_priority: 4,
            units: vec![],
            slice: String::new(),
            boot_ids: vec![],
            limit: 100,
            transports: vec!["syslog".into(), "journal".into(), "stdout".into()],
            date_less_than: 0,
            date_more_than: 0,
            quick_search: String::new(),
            reset_position: true,
        };

        let mut qb = QueryBuilder { query };

        qb.with_default_fields();

        qb
    }

    pub fn with_default_fields(&mut self) -> &mut Self {
        self.query.fields.clear();

        self.query.fields.extend([
            journal_fields::MESSAGE.to_owned(),
            journal_fields::PRIORITY.to_owned(),
            journal_fields::ERRNO.to_owned(),
            journal_fields::SOURCE_REALTIME_TIMESTAMP.to_owned(),
            journal_fields::PID.to_owned(),
            journal_fields::UID.to_owned(),
            journal_fields::COMM.to_owned(),
            journal_fields::SYSTEMD_SLICE.to_owned(),
            journal_fields::SYSTEMD_UNIT.to_owned(),
            journal_fields::SYSTEMD_CGROUP.to_owned(),
            journal_fields::BOOT_ID.to_owned(),
            journal_fields::TRANSPORT.to_owned(),
        ]);

        self
    }

    pub fn with_fields(&mut self, fields: Vec<String>) -> &mut Self {
        self.query.fields.clear();

        self.query.fields.extend(fields);

        self
    }

    pub fn with_transports(&mut self, transports: Vec<String>) -> &mut Self {
        self.query.transports = transports;
        self
    }

    pub fn with_pid(&mut self, pid: u32) -> &mut Self {
        self.query.pid = pid;
        self
    }

    pub fn with_limit(&mut self, limit: u64) -> &mut Self {
        self.query.limit = limit;
        self
    }

    pub fn with_quick_search(&mut self, quick_search: String) -> &mut Self {
        self.query.quick_search = quick_search.to_lowercase();
        self
    }

    pub fn reset_position(&mut self, reset_position: bool) -> &mut Self {
        self.query.reset_position = reset_position;
        self
    }

    pub fn with_date_less_than(&mut self, from_epoch: u64) -> &mut Self {
        self.query.date_less_than = from_epoch;
        self
    }

    pub fn with_date_more_than(&mut self, to_epoch: u64) -> &mut Self {
        self.query.date_more_than = to_epoch;
        self
    }

    pub fn with_priority_above_or_equal_to(&mut self, minimum_priority: u32) -> &mut Self {
        if minimum_priority > 7 {
            self.query.minimum_priority = 7;
        } else {
            self.query.minimum_priority = minimum_priority;
        }

        self
    }

    pub fn with_units(&mut self, units: Vec<String>) -> &mut Self {
        self.query.units = units;
        self
    }

    pub fn within_slice(&mut self, slice: &str) -> &mut Self {
        self.query.slice = String::from(slice);
        self
    }

    pub fn with_boot_ids(&mut self, boot_ids: Vec<String>) -> &mut Self {
        self.query.boot_ids = boot_ids;
        self
    }

    pub fn build(&mut self) -> Query {
        let qb = QueryBuilder::default();
        let old_qb = mem::replace(self, qb);

        old_qb.query
    }
}
