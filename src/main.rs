mod command;
mod config;
mod redis;
mod response;
mod scanner;
mod server;

use crate::config::Config;
use crate::redis::Redis;
use anyhow::Result;
use command::Command;
use server::Server;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Spinning up...");

    let args: Vec<String> = env::args().skip(1).collect();
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let config = Config::from(args.as_slice());
    let redis = Redis::new(config);
    let server = Arc::new(Server::new(redis));
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
