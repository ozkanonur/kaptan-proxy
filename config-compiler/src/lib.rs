#![forbid(unsafe_code)]

use config::Config;

pub mod compiler;
pub mod config;

pub trait Compiler {
    fn read_from_fs() -> Config;
}
