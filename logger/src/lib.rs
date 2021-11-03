#![forbid(unsafe_code)]

pub mod access_log;

#[cfg(not(debug_assertions))]
static ACCESS_LOGS_FILE_PATH: &str = "/var/log/kaptan-proxy/access-logs";

#[cfg(debug_assertions)]
static ACCESS_LOGS_FILE_PATH: &str = ".access-logs";

#[repr(u8)]
pub enum LogLevel {
    /// TODO:
    Off = 0,

    /// TODO:
    All = 1,

    /// TODO:
    Trace = 2,

    /// TODO:
    Debug = 3,

    /// TODO:
    Info = 4,

    /// TODO:
    Warn = 5,

    /// TODO:
    Error = 6,
}

impl LogLevel {
    pub fn from_u8(value: u8) -> LogLevel {
        match value {
            1 => LogLevel::All,
            2 => LogLevel::Trace,
            3 => LogLevel::Debug,
            4 => LogLevel::Info,
            5 => LogLevel::Warn,
            6 => LogLevel::Error,
            _ => LogLevel::Off,
        }
    }
}

pub trait LogCapabilities {
    /// Writes the present instance's **log_message**
    /// to the file system with system's local time.
    ///
    /// # Panics
    /// If the read/write permissions are missing on the
    /// log file.
    ///
    /// # Usage
    /// ```ignore
    /// use logger::{access_log::AccessLog, LogCapabilities};
    ///
    /// fn main() {
    ///     AccessLog { log_message: &[0, 0, 0, 1] }.write();
    /// }
    ///
    /// ```
    fn write(&self);
}
