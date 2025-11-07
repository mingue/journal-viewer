/// message id
pub const MESSAGE_ID: &str = "MESSAGE_ID";
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

/// unit is used for filtering
pub const UNIT_FILTER: &str = "UNIT";
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
