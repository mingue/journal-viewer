use chrono::offset;

pub mod fields {
    /// human-readable message string for this entry
    pub const MESSAGE: &str = "MESSAGE";
    /// priority value between 0 ("emerg") and 7 ("debug")
    pub const PRIORITY: &str = "PRIORITY";
    /// low-level Unix error number causing this entry, if any
    pub const ERRNO: &str = "ERRNO";
    /// This is the time in microseconds since the epoch UTC, formatted as a decimal string
    pub const SOURCE_REALTIME_TIMESTAMP: &str = "_SOURCE_REALTIME_TIMESTAMP";

    /// The process ID of the process the journal entry originates from
    pub const PID: &str = "_PID";
    /// The user ID of the process the journal entry originates from
    pub const UID: &str = "_UID";
    /// The group ID of the process the journal entry originates from
    pub const GID: &str = "_GID";

    /// The name of the process
    pub const COMM: &str = "_COMM";
    /// the executable path
    pub const EXE: &str = "_EXE";
    /// command line of the process
    pub const CMDLINE: &str = "_CMDLINE";

    /// the systemd slice unit name
    pub const SYSTEMD_SLICE: &str = "_SYSTEMD_SLICE";
    /// the systemd unit name
    pub const SYSTEMD_UNIT: &str = "_SYSTEMD_UNIT";
    /// The control group path in the systemd hierarchy
    pub const SYSTEMD_CGROUP: &str = "_SYSTEMD_CGROUP";

    /// The kernel boot ID
    pub const BOOT_ID: &str = "_BOOT_ID";

    /// How the entry was received by the journal service
    /// Valid transports are:
    ///   audit: for those read from the kernel audit subsystem
    ///   driver: for internally generated messages
    ///   syslog: for those received via the local syslog socket
    ///   journal: for those received via the native journal protocol
    ///   stdout: for those read from a service's standard output or error output
    ///   kernel: for those read from the kernel
    pub const TRANSPORT: &str = "_TRANSPORT";
}
pub struct QueryBuilder {
    pub(crate) pid: u32,
    pub(crate) fields: Vec<String>,
    pub(crate) minimum_priority: u32,
    pub(crate) unit: String,
    pub(crate) slice: String,
    pub(crate) boot_id: String,
    pub(crate) limit: u64,
    pub(crate) skip: u64,
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
        };

        qb.with_default_fields();

        qb
    }

    pub fn with_default_fields(&mut self) -> &mut Self {
        self.fields.clear();

        self.fields.extend([
            fields::MESSAGE.to_owned(),
            fields::PRIORITY.to_owned(),
            fields::ERRNO.to_owned(),
            fields::SOURCE_REALTIME_TIMESTAMP.to_owned(),
            fields::PID.to_owned(),
            fields::UID.to_owned(),
            fields::COMM.to_owned(),
            fields::SYSTEMD_SLICE.to_owned(),
            fields::SYSTEMD_UNIT.to_owned(),
            fields::SYSTEMD_CGROUP.to_owned(),
            fields::BOOT_ID.to_owned(),
            fields::TRANSPORT.to_owned(),
        ]);

        self
    }

    pub fn with_fields(&mut self, fields: Vec<String>) -> &mut Self {
        self.fields.clear();

        self.fields.extend(fields);

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

    pub fn with_priority_above(&mut self, minimum_priority: u32) -> Result<&mut Self, &str> {
        if minimum_priority > 7 {
            return Err("The highest priority is 7");
        }

        self.minimum_priority = minimum_priority;

        Ok(self)
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
