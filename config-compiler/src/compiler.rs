use crate::{config::Config, Compiler};
use std::fs;

impl Compiler for Config {
    fn read_from_fs() -> Config {
        let config_path: &str = "/etc/kaptan-proxy/cfg.toml";
        let config_str = fs::read_to_string(config_path)
            .unwrap_or_else(|_| panic!("{} could not found.", config_path));

        let decoded_config: Config = toml::from_str(&config_str)
            .unwrap_or_else(|_| panic!("Failed to parse configuration file."));

        decoded_config
    }
}

#[test]
fn test_write() {
    Config::read_from_fs();
}
