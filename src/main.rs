mod command;
mod response;

use anyhow::Result;
use bytes::BytesMut;
use command::Command;
use response::{Builder, Response};
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
        tokio::spawn(async move {
            let result = handle_connection(stream).await;
            match result {
                Ok(_) => println!("Served connection from: {origin}"),
                Err(e) => println!("Failed serving {origin} with error: {e}"),
            };
        });
    }
}

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let mut stream = stream;
    let mut buffer = BytesMut::with_capacity(1024);

    loop {
        let read_size = stream.read_buf(&mut buffer).await?;
        if read_size == 0 {
            println!("Empty message, shutting down connection.");
            stream.shutdown().await?;
            return Ok(());
        }

        let frame = std::str::from_utf8(&buffer)?;
        println!("Received message: {frame}");

        let mut lines = frame.lines();
        let command = Command::try_from(&mut lines);
        let response = handle_command(command);

        stream.write_all(response.as_bytes()).await?;
    }
}

fn handle_command(c: Result<Command, String>) -> Response {
    match c {
        Ok(Command::Ping) => Response::pong(),
        Ok(Command::Echo(message)) => Response::text(&message),
        Err(e) => Response::error(&e),
    }
}
