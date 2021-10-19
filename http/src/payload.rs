use crate::ParseRawTcpBuffer;

#[derive(Debug)]
pub struct HttpQualifications {
    pub method: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
    pub host: Option<String>,
}

impl ParseRawTcpBuffer for HttpQualifications {
    fn parse(buffer: &String) -> HttpQualifications {
        let mut buffer_to_lines = buffer.lines();

        let method_path_version: Vec<&str> = buffer_to_lines
            .next()
            .unwrap()
            .split(" ")
            .collect();

        let host: Vec<&str> = buffer_to_lines.next().unwrap().split("Host: ").collect();

        HttpQualifications {
            method: Some(method_path_version[0].to_string()),
            path: Some(method_path_version[1].to_string()),
            version: Some(method_path_version[2].to_string()),
            host: Some(host[1].to_string()),
        }
    }
}

