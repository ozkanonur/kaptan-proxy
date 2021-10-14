use crate::{config::Config, Compiler};
use std::fs;

impl Compiler for Config {
    fn read_from_fs() -> Config {
        let config_path: &str = "/etc/kaptan-proxy/cfg.toml";
        let config_str = fs::read_to_string(config_path)
            .expect(&format!("{} could not found.", config_path).to_owned());

        let decoded_config: Config = toml::from_str(&config_str).unwrap();

        decoded_config
    }
}
