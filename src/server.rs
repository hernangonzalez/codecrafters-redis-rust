use crate::redis::Redis;
use crate::response::{Builder, Response};
use crate::scanner;
use anyhow::Result;
use bytes::BytesMut;
use std::time;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Server {
    redis: Redis,
}

impl Server {
    pub fn new() -> Self {
        Self {
            redis: Redis::new(),
        }
    }

    pub async fn handle_connection(&self, stream: TcpStream) -> Result<()> {
        let mut stream = stream;
        let mut buffer = BytesMut::with_capacity(1024);
        let now = time::Instant::now();

        loop {
            let read_size = stream.read_buf(&mut buffer).await?;
            if read_size == 0 {
                println!("Empty message, shutting down connection.");
                stream.shutdown().await?;
                return Ok(());
            }

            let frame = std::str::from_utf8(&buffer)?;
            let commands = scanner::scan(frame);
            println!("Received #{} commands - at {now:?}", commands.len());

            let responses = commands
                .iter()
                .filter_map(|c| self.redis.handle(c, now))
                .collect::<Vec<_>>();

            if responses.is_empty() {
                flush(&mut stream, &Response::error("No supported command found")).await?;
            } else {
                flush_all(&mut stream, &responses).await?;
            }
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
