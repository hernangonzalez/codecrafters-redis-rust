mod command;
mod response;
mod scanner;

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
        let commands = scanner::scan(frame);
        println!("Received #{} commands", commands.len());

        let responses = commands
            .iter()
            .filter_map(into_response)
            .collect::<Vec<_>>();

        if responses.is_empty() {
            flush(&mut stream, &Response::error("No supported command found")).await?;
        } else {
            flush_all(&mut stream, &responses).await?;
        }
    }
}

async fn flush(stream: &mut TcpStream, response: &Response) -> Result<()> {
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

async fn flush_all(stream: &mut TcpStream, all: &[Response]) -> Result<()> {
    for response in all {
        flush(stream, response).await?;
    }
    Ok(())
}

fn into_response(cmd: &Command) -> Option<Response> {
    match cmd {
        Command::Ping => Some(Response::pong()),
        Command::Echo(message) => Some(Response::text(message)),
        Command::Unknown(cmd, args) => {
            println!("Skip unknown command: {cmd}, args: {args}");
            None
        }
    }
}
