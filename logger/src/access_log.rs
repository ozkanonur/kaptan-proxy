use crate::LogCapabilities;
use chrono::Local;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

pub struct AccessLog<'a, S> {
    pub request: &'a S,
}

impl<S> LogCapabilities for AccessLog<'_, S>
where
    S: Debug,
{
    fn write(&self) {
        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(false)
            .open("access-logs")
            .expect("Unable to open file");

        let mut formatted_bytes = Vec::new();
        writeln!(
            &mut formatted_bytes,
            "{} -> {:?}",
            Local::now().naive_local(),
            self.request
        )
        .unwrap();

        log_file
            .write_all(&formatted_bytes)
            .expect("Panic raised while writing access log to the file.");
    }
}
