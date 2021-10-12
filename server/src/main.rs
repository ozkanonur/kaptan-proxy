#![warn(rust_2018_idioms)]

use config_compiler::compiler::*;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::error::Error;

mod runtime;

fn main() {
    let config = get_configuration();
    println!("{:?}", config);

    runtime::create().block_on(async move {
        let listen_addr = format_args!("127.0.0.1:{}", config.runtime_config.inbound_port).to_string();
        let server_addr = config.runtime_config.outbound_addr;

        println!("Listening on: {}", listen_addr);
        println!("Proxying to: {}", server_addr);

        let listener = TcpListener::bind(listen_addr).await.unwrap();
        while let Ok((inbound, _)) = listener.accept().await {
            let transfer = transfer(inbound, server_addr.clone()).map(|r| {
                if let Err(e) = r {
                    println!("Failed to transfer; error={}", e);
                }
            });

            tokio::spawn(transfer);
        }
    });
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}
