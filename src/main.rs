mod command;
mod redis;
mod response;
mod scanner;
mod server;

use anyhow::Result;
use command::Command;
use server::Server;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Spinning up...");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let server = Arc::new(Server::new());
    println!("Listening at 6379...");

    loop {
        let (stream, origin) = listener.accept().await?;
        let server = Arc::clone(&server);
        println!("Client connected from: {origin}");
        tokio::spawn(async move {
            let result = server.handle_connection(stream).await;
            match result {
                Ok(_) => println!("Served connection from: {origin}"),
                Err(e) => println!("Failed serving {origin} with error: {e}"),
            };
        });
    }
}
