#![forbid(unsafe_code)]

use config::Config;

pub mod compiler;
pub mod config;

pub trait Compiler {
    /// Reads the configurations from /etc/kaptan-proxy/cfg.toml, deserializes
    /// it into config::Config struct and returns an instance of it.
    ///
    /// # Panics
    /// If:
    ///     - Configuration value types are incorrect.
    ///     - **/etc/kaptan-proxy/cfg.toml** does not exists.
    ///     - Missing permission for **/etc/kaptan-proxy/cfg.toml**.
    ///
    /// # Usage
    /// ```
    /// use config_compiler::{config::Config, Compiler};
    ///
    /// fn main*( {
    ///     let config = Config::read_from_fs();
    /// }
    ///
    /// ```
    fn read_from_fs() -> Config;
}
