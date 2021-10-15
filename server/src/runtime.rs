use config_compiler::config::*;
use tokio::runtime::{Builder, Runtime};

#[cfg(not(feature = "concurrency"))]
/// - Creates and returns a single-threaded instance of tokio::runtime::Runtime.
/// - Will be used compiled only if the 'concurrency' feature is disabled.
pub(crate) fn create(_: &Config) -> Runtime {
    Builder::new_current_thread()
        .enable_all()
        .thread_name("proxy_thread_space")
        .build()
        .expect("An unexpected error has occurred on creating single-thread runtime.")
}

#[cfg(feature = "concurrency")]
/// - Creates and returns an instance of tokio::runtime::Runtime.
/// - Size of Thread Pool is determined by given RuntimeConfig.
/// - Will be compiled only if the 'concurrency' feature is enabled.
pub(crate) fn create(config: &RuntimeConfig) -> Runtime {
    let cpus = num_cpus::get();

    if config.worker_threads > 0 {
        println!(
            "{} cpus will be used at runtime",
            config.worker_threads
        );
    } else {
        println!("{} cpus will be used at runtime", cpus);
    }

    match ThreadModel::from_usize(config.worker_threads) {
        ThreadModel::Single => Builder::new_current_thread()
            .enable_all()
            .thread_name("proxy_thread_space")
            .build()
            .expect("An unexpected error has occurred on creating single-thread runtime."),
        ThreadModel::Default => Builder::new_multi_thread()
            .enable_all()
            .thread_name("proxy_thread_space")
            .worker_threads(cpus)
            .max_blocking_threads(cpus)
            .build()
            .expect("An unexpected error has occurred on creating multi-thread runtime."),
        ThreadModel::Multi => Builder::new_multi_thread()
            .enable_all()
            .thread_name("proxy_thread_space")
            .worker_threads(config.worker_threads)
            .max_blocking_threads(config.worker_threads)
            .build()
            .expect("An unexpected error has occurred on creating multi-thread runtime."),
    }
}

