//! Data collection for processes.
//!
//! For Linux, this is handled by a custom set of functions.
//! For Windows and macOS, this is handled by sysinfo.

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub use self::linux::*;
    } else if #[cfg(target_os = "macos")] {
        pub mod macos;
        pub use self::macos::*;
    } else if #[cfg(target_os = "windows")] {
        pub mod windows;
        pub use self::windows::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_family = "unix")] {
        pub mod unix;
        pub use self::unix::*;
    }
}

use crate::Pid;

// TODO: Add value so we know if it's sorted ascending or descending by default?
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ProcessSorting {
    CpuPercent,
    Mem,
    MemPercent,
    Pid,
    ProcessName,
    Command,
    ReadPerSecond,
    WritePerSecond,
    TotalRead,
    TotalWrite,
    State,
    User,
    Count,
}

impl std::fmt::Display for ProcessSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                ProcessSorting::CpuPercent => "CPU%",
                ProcessSorting::MemPercent => "Mem%",
                ProcessSorting::Mem => "Mem",
                ProcessSorting::ReadPerSecond => "R/s",
                ProcessSorting::WritePerSecond => "W/s",
                ProcessSorting::TotalRead => "T.Read",
                ProcessSorting::TotalWrite => "T.Write",
                ProcessSorting::State => "State",
                ProcessSorting::ProcessName => "Name",
                ProcessSorting::Command => "Command",
                ProcessSorting::Pid => "PID",
                ProcessSorting::Count => "Count",
                ProcessSorting::User => "User",
            }
        )
    }
}

impl Default for ProcessSorting {
    fn default() -> Self {
        ProcessSorting::CpuPercent
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProcessHarvest {
    /// The pid of the process.
    pub pid: Pid,

    /// The parent PID of the process. Remember, parent_pid 0 is root.
    pub parent_pid: Option<Pid>,

    /// CPU usage as a percentage.
    pub cpu_usage_percent: f64,

    /// Memory usage as a percentage.
    pub mem_usage_percent: f64,

    /// Memory usage as bytes.
    pub mem_usage_bytes: u64,

    /// The name of the process.
    pub name: String,

    /// The exact command for the process.
    pub command: String,

    /// Bytes read per second.
    pub read_bytes_per_sec: u64,

    /// Bytes written per second.
    pub write_bytes_per_sec: u64,

    /// The total number of bytes read by the process.
    pub total_read_bytes: u64,

    /// The total number of bytes written by the process.
    pub total_write_bytes: u64,

    /// The current state of the process (e.g. zombie, asleep)
    pub process_state: String,

    /// The process state represented by a character. TODO: Merge with above as a single struct.
    pub process_state_char: char,

    /// This is the *effective* user ID of the process.
    #[cfg(target_family = "unix")]
    pub uid: Option<libc::uid_t>,
    // pub rss_kb: u64,
    // pub virt_kb: u64,
}
