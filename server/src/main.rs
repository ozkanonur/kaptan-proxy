#![warn(rust_2018_idioms)]

use bytes::Bytes;
use config_compiler::compiler::*;
use futures::{
    self,
    stream::{Stream, StreamExt, TryStreamExt},
    FutureExt,
};
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncRead, Result};
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
            let transfer = transfer(inbound, server_addr.clone()).map(|r| {
                if let Err(e) = r {
                    println!("Failed to transfer; error={}", e);
                }
            });

            tokio::spawn(transfer);
        }
    });
}

fn byte_to_stream<R>(r: R) -> impl Stream<Item = Result<Bytes>>
where
    R: AsyncRead,
{
    FramedRead::new(r, BytesCodec::new()).map_ok(|bytes| bytes.freeze())
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<()> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (read_inbound, mut write_inbound) = inbound.split();
    let (mut read_outbound, mut write_outbound) = outbound.split();

    let cli_read_stream = byte_to_stream(read_inbound).map(|buf| {
        println!("{:?}", buf);
        buf
    });

    let client_to_server = async {
        let mut read_inbound = StreamReader::new(cli_read_stream);

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
