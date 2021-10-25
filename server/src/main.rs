#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

use config_compiler::{config::Config, Compiler};
use middlewares::logging_middleware::LoggingMiddleware;

use proxy::{service::ProxyService, Http, ServiceBuilder};

use tokio::net::TcpListener;

mod runtime;

fn main() {
    let config = Config::read_from_fs();
    let routes = config.proxy.clone();
    let listen_addr = format_args!("127.0.0.1:{}", config.runtime.inbound_port).to_string();

    runtime::create(&config.runtime).block_on(async move {
        let tcp_listener = TcpListener::bind(listen_addr).await.unwrap();
        let service_builder = ServiceBuilder::new();

        while let Ok((tcp_stream, _)) = tcp_listener.accept().await {
            let routes = routes.clone();
            let service_builder = service_builder.clone();

            tokio::spawn(async move {
                if let Err(http_err) = Http::new()
                    .http1_keep_alive(true)
                    .serve_connection(
                        tcp_stream,
                        service_builder.service(LoggingMiddleware::new(ProxyService { routes })),
                    )
                    .await
                {
                    eprintln!("HTTP exception -> {}", http_err);
                }
            });
        }
    });
}

