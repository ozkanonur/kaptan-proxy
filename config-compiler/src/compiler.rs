use crate::config;

pub fn get_configuration() -> config::Config {
    // TODO:
    // Compile the config file from /etc/kaptan/kaptan.cfg

    config::Config::default()
}
