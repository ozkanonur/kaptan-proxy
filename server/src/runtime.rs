use config_compiler::config::*;
use tokio::runtime::{Builder, Runtime};

/// - Creates and returns an instance of tokio::runtime::Runtime.
/// - Size of Thread Pool is determined by given RuntimeConfig.
pub(crate) fn create(config: &RuntimeConfig) -> Runtime {
    let cpus = num_cpus::get();

    match ThreadModel::from_usize(config.worker_threads) {
        ThreadModel::Default => {
            println!("{} cpus will be used at runtime", cpus);
            Builder::new_multi_thread()
                .enable_all()
                .thread_name("proxy_thread_space")
                .worker_threads(cpus)
                .max_blocking_threads(cpus)
                .build()
                .expect("An unexpected error has occurred on creating multi-thread runtime.")
        }
        ThreadModel::Single => {
            println!("{} cpus will be used at runtime", config.worker_threads);
            Builder::new_current_thread()
                .enable_all()
                .thread_name("proxy_thread_space")
                .build()
                .expect("An unexpected error has occurred on creating single-thread runtime.")
        }
        ThreadModel::Multi => {
            println!("{} cpus will be used at runtime", config.worker_threads);
            Builder::new_multi_thread()
                .enable_all()
                .thread_name("proxy_thread_space")
                .worker_threads(config.worker_threads)
                .max_blocking_threads(config.worker_threads)
                .build()
                .expect("An unexpected error has occurred on creating multi-thread runtime.")
        }
    }
}
