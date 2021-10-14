#[repr(usize)]
pub enum ThreadModel {
    Default = 0,
    Single = 1,
    Multi,
}

impl ThreadModel {
    pub fn from_usize(value: usize) -> ThreadModel {
        match value {
            0 => ThreadModel::Default,
            1 => ThreadModel::Single,
            _ => ThreadModel::Multi,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub runtime: RuntimeConfig,
}

#[derive(Debug, serde::Deserialize)]
pub struct RuntimeConfig {
    pub worker_cores: usize,
    pub inbound_port: u32,
    pub outbound_addr: String,
    pub log_level: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            runtime: RuntimeConfig {
                worker_cores: 0, // 0 means use all the available threads
                inbound_port: 6150,
                outbound_addr: "127.0.0.1:8080".to_string(),
                log_level: 0,
            },
        }
    }
}
