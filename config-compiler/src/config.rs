#[derive(Debug)]
pub struct Config {
    pub runtime_config: RuntimeConfig,
}

#[derive(Debug)]
pub struct RuntimeConfig {
    pub worker_cores: usize,
    pub inbound_port: u32,
    pub outbound_addr: String,
    pub log_level: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            runtime_config: RuntimeConfig {
                worker_cores: 0, // 0 means use all the available threads
                inbound_port: 6150,
                outbound_addr: "127.0.0.1:8080".to_string(),
                log_level: 0,
            },
        }
    }
}
