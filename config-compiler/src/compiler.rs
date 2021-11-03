use crate::{config::Config, Compiler};
use std::fs;

impl Compiler for Config {
    fn read_from_fs() -> Config {
        let config_str = fs::read_to_string(super::CFG_PATH)
            .unwrap_or_else(|_| panic!("{} could not found.", super::CFG_PATH));

        let decoded_config: Config = toml::from_str(&config_str)
            .unwrap_or_else(|_| panic!("Failed to parse configuration file."));

        decoded_config
    }
}

#[test]
fn test_write() {
    Config::read_from_fs();
}
