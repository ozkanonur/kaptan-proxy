use crate::LogCapabilities;
use chrono::Local;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

pub struct AccessLog<'a, X, Y, Z, T> {
    pub method: &'a X,
    pub uri: &'a Y,
    pub version: &'a Z,
    pub headers: &'a T,
}

impl<X, Y, Z, T> LogCapabilities for AccessLog<'_, X, Y, Z, T>
where
    X: Debug,
    Y: Debug,
    Z: Debug,
    T: Debug,
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
            "{{\"_time\": \"{}\", \"method\": \"{:?}\", \"uri\": \"{:?}\", \"version\": \"{:?}\", \"headers\": {:?}}}",
            Local::now().naive_local(),
            self.method,
            self.uri,
            self.version,
            self.headers
        )
        .unwrap();

        log_file
            .write_all(&formatted_bytes)
            .expect("Panic raised while writing access log to the file.");
    }
}
