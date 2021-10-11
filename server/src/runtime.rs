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
    let mut cores = std::env::var("CORES")
        .ok()
        .and_then(|v| {
            let opt = v.parse::<usize>().ok().filter(|n| *n > 0);
            if opt.is_none() {
                println!("CORES is not defined, the default workflow will be processed.");
            }
            opt
        })
        .unwrap_or(0);

    let cpus = num_cpus::get();
    println!("{} cores are running", cpus);
    debug_assert!(cpus > 0, "No available cpu was found");

    if cores > cpus {
        cores = cpus;
    }

    match cores {
        0 | 1 => Builder::new_current_thread()
            .enable_all()
            .thread_name("proxy_thread_space")
            .build()
            .expect("An unexpected error has occurred on creating single-thread runtime."),
        num_cpus => Builder::new_multi_thread()
            .enable_all()
            .thread_name("proxy_thread_space")
            .worker_threads(num_cpus)
            .max_blocking_threads(num_cpus)
            .build()
            .expect("An unexpected error has occurred on creating multi-thread runtime."),
    }
}

