// Uncomment this block to pass the first stage
use anyhow::*;
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Framed};

#[tokio::main]
async fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Spinning up...");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Listening at 6379...");

    loop {
        let (stream, origin) = listener.accept().await?;
        println!("Client connected from: {origin}");

        let mut framed = Framed::new(stream, BytesCodec::new());
        if let Some(message) = framed.next().await {
            let message = message?;
            println!("Received message: {}", String::from_utf8_lossy(&message));
            framed.into_inner().write_all(b"+PONG\r\n").await?;
        };
    }
}
