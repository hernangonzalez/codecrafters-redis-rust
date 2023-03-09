// Uncomment this block to pass the first stage
use anyhow::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Spinning up...");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Listening at 6379...");

    loop {
        let (stream, origin) = listener.accept().await?;
        println!("Client connected from: {origin}");
        handle_connection(stream).await?;
    }
}

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let mut stream = stream;
    let mut buffer = [0u8; 1024];

    loop {
        let read_size = stream.read(&mut buffer).await?;
        if read_size == 0 {
            println!("Empty message, shutting down");
            stream.shutdown().await?
        }

        println!("Received message: {}", String::from_utf8_lossy(&buffer));
        let response = match &buffer[..] {
            b"PING" => "+PONG\r\n",
            _ => "+PONG\r\n",
        };

        stream.write_all(response.as_bytes()).await?;
    }
}
