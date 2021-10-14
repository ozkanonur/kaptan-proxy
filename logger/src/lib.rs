use access_log::AccessLog;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

pub mod access_log;

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
