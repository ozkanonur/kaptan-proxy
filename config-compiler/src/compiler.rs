use std::fs;
use crate::config::Config;

pub fn get_configuration() -> Config {
    let config_path: &str = "/etc/kaptan/cfg.toml";
    let config_str = fs::read_to_string(config_path)
        .expect(&format!("{} could not found.", config_path).to_owned());

    let decoded_config: Config = toml::from_str(&config_str).unwrap();

    decoded_config
}
