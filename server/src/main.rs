#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

use config_compiler::compiler::*;
use futures::{stream::TryStreamExt, FutureExt};

use logger::LogLevel;
use logger::{access_log::AccessLog, LogCapabilities};
use tokio::io;
use tokio::io::{AsyncWriteExt, Result};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, FramedRead};
use tokio_util::io::StreamReader;
mod runtime;

fn main() {
    let config = get_configuration();
    println!("{:?}", config);

    runtime::create(&config).block_on(async move {
        let listen_addr = format_args!("127.0.0.1:{}", config.runtime.inbound_port).to_string();
        let server_addr = config.runtime.outbound_addr;

        println!("Listening on: {}", listen_addr);
        println!("Proxying to: {}", server_addr);

        let listener = TcpListener::bind(listen_addr).await.unwrap();
        while let Ok((inbound, _)) = listener.accept().await {
            let transfer = transfer(
                inbound,
                server_addr.clone(),
                LogLevel::from_u8(config.runtime.log_level),
            )
            .map(|r| {
                if let Err(e) = r {
                    println!("Failed to transfer; error={}", e);
                }
            });

            tokio::spawn(transfer);
        }
    });
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String, log_level: LogLevel) -> Result<()> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (read_inbound, mut write_inbound) = inbound.split();
    let (mut read_outbound, mut write_outbound) = outbound.split();

    let frames = FramedRead::new(read_inbound, BytesCodec::new()).map_ok(|buf| {
        match log_level {
            LogLevel::All => {
                AccessLog { log_message: &buf }.write();
            }
            _ => (),
        }

        buf
    });

    let client_to_server = async {
        let mut read_inbound = StreamReader::new(frames);

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
