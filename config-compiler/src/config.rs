#[derive(Debug)]
pub struct Config {
    pub runtime_config: RuntimeConfig,
}

#[derive(Debug)]
pub struct RuntimeConfig {
    pub worker_cores: usize,
    pub inbound_port: u32,
    pub outbound_addr: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            runtime_config: RuntimeConfig {
                worker_cores: 5,
                inbound_port: 6150,
                outbound_addr: "127.0.0.1:8080".to_string(),
            },
        }
    }
}
