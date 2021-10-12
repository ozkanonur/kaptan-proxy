use config_compiler::compiler::*;
use tokio::runtime::{Builder, Runtime};

#[cfg(not(feature = "concurrency"))]
pub(crate) fn create() -> Runtime {
    Builder::new_current_thread()
        .enable_all()
        .thread_name("proxy_thread_space")
        .build()
        .expect("An unexpected error has occurred on creating single-thread runtime.")
}

#[cfg(feature = "concurrency")]
pub(crate) fn create() -> Runtime {
    let config = get_configuration();

    let cpus = num_cpus::get();

    if config.runtime_config.worker_cores > 0 {
        println!("{} cpus will be used at runtime", config.runtime_config.worker_cores);
    } else {
        println!("{} cpus will be used at runtime", cpus);
    }

    match config.runtime_config.worker_cores {
        1 => Builder::new_current_thread()
            .enable_all()
            .thread_name("proxy_thread_space")
            .build()
            .expect("An unexpected error has occurred on creating single-thread runtime."),
        0 => Builder::new_multi_thread()
            .enable_all()
            .thread_name("proxy_thread_space")
            .worker_threads(cpus)
            .max_blocking_threads(cpus)
            .build()
            .expect("An unexpected error has occurred on creating multi-thread runtime."),
        _ => Builder::new_multi_thread()
            .enable_all()
            .thread_name("proxy_thread_space")
            .worker_threads(config.runtime_config.worker_cores)
            .max_blocking_threads(config.runtime_config.worker_cores)
            .build()
            .expect("An unexpected error has occurred on creating multi-thread runtime."),
    }
}

