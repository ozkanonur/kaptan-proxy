#![forbid(unsafe_code)]

use access_log::AccessLog;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

pub mod access_log;

#[repr(u8)]
pub enum LogLevel {
    Off = 0,
    All = 1,
    Trace = 2,
    Debug = 3,
    Info = 4,
    Warn = 5,
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
    fn write(&self);
}

impl LogCapabilities for AccessLog<'_> {
    fn write(&self) {
        let mut log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("access-logs")
            .expect("Unable to open file");

        let formatted_log = String::from_utf8_lossy(&self.log_message)
            .trim()
            .replace("\r\n", "");
        let local_time = Local::now();
        let formatted_log = format!("{} -> {}\n", local_time, formatted_log);
        log_file.write_all(formatted_log.as_bytes()).expect("stuff");
    }
}
