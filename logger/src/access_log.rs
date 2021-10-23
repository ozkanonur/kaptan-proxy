use crate::LogCapabilities;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::fmt::Debug;

pub struct AccessLog<'a, S> {
    pub request: &'a S,
}

impl<S> LogCapabilities for AccessLog<'_, S> where S: Debug {
    fn write(&self) {
        let mut log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("access-logs")
            .expect("Unable to open file");

        writeln!(&mut log_file, "{} -> {:?}", Local::now().naive_local(), self.request).unwrap();
    }
}
