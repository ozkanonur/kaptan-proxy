#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

use config_compiler::{
    config::{Config, RoutesStruct},
    Compiler,
};

use hyper::{server::conn::Http, service::service_fn, Body, Client, Request, Response};
use logger::LogLevel;
use tokio::io;
use tokio::io::{AsyncWriteExt, Result};
use tokio::net::{TcpListener, TcpStream};
mod runtime;

fn main() {
    let config = Config::read_from_fs();
    println!("{:?}", config);

    let listen_addr = format_args!("127.0.0.1:{}", config.runtime.inbound_port).to_string();
    runtime::create(&config.runtime).block_on(async move {
        let tcp_listener = TcpListener::bind(listen_addr).await.unwrap();

        while let Ok((tcp_stream, _)) = tcp_listener.accept().await {
            tokio::task::spawn(async move {
                if let Err(http_err) = Http::new()
                    .http1_keep_alive(true)
                    .serve_connection(tcp_stream, service_fn(move |req| hello(req, "testing-stuff")))
                    .await
                {
                    eprintln!("Error while serving HTTP connection: {}", http_err);
                }
            });
        }
    });
}

async fn hello(mut req: Request<Body>, path: &str) -> Result<Response<Body>> {
    println!("{}", path);
    let client = Client::new();
    let uri_string = format!(
        "http://{}{}",
        "127.0.0.1:8080",
        req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
    );

    let uri = uri_string.parse().unwrap();
    *req.uri_mut() = uri;
    Ok(client.request(req).await.unwrap())
}

/// Simply proxies inbound connection to outbound connection.
/// Returns Result<()>.
///
/// # Panics
/// No panic scenario implemented yet. However, logs
/// the errors of destination address is unreachable.
async fn transfer(
    mut inbound: TcpStream,
    _routes: Option<Vec<RoutesStruct>>,
    _log_level: LogLevel,
) -> Result<()> {
    let (mut read_inbound, mut write_inbound) = inbound.split();

    let mut outbound = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut read_outbound, mut write_outbound) = outbound.split();

    let client_to_server = async {
        io::copy(&mut read_inbound, &mut write_outbound).await?;
        write_outbound.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut read_outbound, &mut write_inbound).await?;
        write_inbound.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}

