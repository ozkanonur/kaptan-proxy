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
            .open(super::ACCESS_LOGS_FILE_PATH)
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

#[test]
fn test_write() {
    #[derive(Debug)]
    pub struct HeaderMock {
        pub key: String,
        pub value: Option<String>,
    }

    let mut headers: Vec<HeaderMock> = Vec::new();

    headers.push(HeaderMock {
        key: "host".to_string(),
        value: Some("127.0.0.1".to_string()),
    });

    AccessLog {
        method: &"GET",
        uri: &"/test/write",
        version: &"HTTP/1.1",
        headers: &headers,
    }
    .write();
}
