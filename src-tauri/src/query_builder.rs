use crate::journal_fields;

pub struct QueryBuilder {
    pub(crate) pid: u32,
    pub(crate) fields: Vec<String>,
    pub(crate) minimum_priority: u32,
    pub(crate) unit: String,
    pub(crate) slice: String,
    pub(crate) boot_id: String,
    pub(crate) limit: u64,
    pub(crate) skip: u64,
    pub(crate) transports: Vec<String>,
}

impl QueryBuilder {
    pub fn default() -> Self {
        let mut qb = QueryBuilder {
            pid: 0,
            fields: vec![],
            minimum_priority: 4,
            unit: String::new(),
            slice: String::new(),
            boot_id: String::new(),
            limit: 100,
            skip: 0,
            transports: vec![
                "driver".into(),
                "syslog".into(),
                "journal".into(),
                "stdout".into(),
            ],
        };

        qb.with_default_fields();

        qb
    }

    pub fn with_default_fields(&mut self) -> &mut Self {
        self.fields.clear();

        self.fields.extend([
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
        self.fields.clear();

        self.fields.extend(fields);

        self
    }

    pub fn with_transports(&mut self, transports: Vec<String>) -> &mut Self {
        self.transports = transports;
        self
    }

    pub fn with_pid(&mut self, pid: u32) -> &mut Self {
        self.pid = pid;
        self
    }

    pub fn with_offset(&mut self, offset: u64) -> &mut Self {
        self.skip = offset;
        self
    }

    pub fn with_limit(&mut self, limit: u64) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn with_priority_above_or_equal_to(&mut self, minimum_priority: u32) -> &mut Self {
        if minimum_priority > 7 {
            self.minimum_priority = 7;
        } else {
            self.minimum_priority = minimum_priority;
        }

        self
    }

    pub fn with_unit(&mut self, unit: &str) -> &mut Self {
        let mut full_unit: String = String::from(unit);
        if !full_unit.contains(".") {
            full_unit.push_str(".service");
        }

        self.unit = full_unit;
        self
    }

    pub fn within_slice(&mut self, slice: &str) -> &mut Self {
        self.slice = String::from(slice);
        self
    }

    pub fn with_boot_id(&mut self, boot_id: &str) -> &mut Self {
        self.boot_id = String::from(boot_id);
        self
    }
}
