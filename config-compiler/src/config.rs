#[repr(usize)]
pub enum ThreadModel {
    /// Consumes all available cores. The number of available
    /// cores is determined by checking the environment
    /// or by inspecting the host or cgroups.
    Default = 0,

    /// Runs everything on the current thread. So single-thread.
    Single = 1,

    /// Creates a thread pool based on the given number.
    /// This thread pool will be used at runtime.
    Multi,
}

impl ThreadModel {
    pub fn from_usize(value: usize) -> ThreadModel {
        match value {
            0 => ThreadModel::Default,
            1 => ThreadModel::Single,
            _ => ThreadModel::Multi,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    /// Provides necessary configurations that runtime needs.
    pub runtime: RuntimeConfig,
    /// Defines list of routes with their spesific destination addresses.
    pub target: Target,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RuntimeConfig {
    /// Specifies the number of threads to run in the runtime.
    ///
    /// Default: 0 # Means use all the available threads
    pub worker_threads: usize,

    /// Specifies port of the proxy server that will be listening on.
    ///
    /// Default: 6150
    pub inbound_port: u32,

    /// Specifies the logging level/profile of the runtime.
    ///
    /// # Levels
    /// 0: Off
    /// 1: All
    /// 2: Trace
    /// 3: Debug
    /// 4: Info
    /// 5: Warn
    /// 6: Error
    ///
    /// Default: 0
    pub log_level: u8,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Target {
    /// Specifies the routes and the address to which they will be proxied.
    ///
    /// Default: None
    pub routes: Option<Vec<RoutesStruct>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RoutesStruct {
    /// Route that will proxy to destionation address.
    pub route: String,
    /// Target that will be proxied from the specified route.
    pub target: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            runtime: RuntimeConfig {
                worker_threads: 0,
                inbound_port: 6150,
                log_level: 0,
            },
            target: Target { routes: None },
        }
    }
}
