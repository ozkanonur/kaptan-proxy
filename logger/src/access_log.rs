use crate::LogCapabilities;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

pub struct AccessLog {
    pub log_message: String,
}

impl AccessLog {}

impl LogCapabilities for AccessLog {
    fn write(&self) {
        let mut log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("access-logs")
            .expect("Unable to open file");

        let formatted_log = &self.log_message.trim().replace("\r\n", " | ");
        let local_time = Local::now();
        let formatted_log = format!("{} -> {}\n", local_time, formatted_log);
        log_file.write_all(formatted_log.as_bytes()).expect("stuff");
    }
}
