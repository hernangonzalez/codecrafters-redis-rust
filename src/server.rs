use crate::{
    redis::Redis,
    response::{Builder, Response},
    scanner,
};
use anyhow::Result;
use bytes::BytesMut;
use std::time;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct Server {
    redis: Redis,
}

impl Server {
    pub fn new(redis: Redis) -> Self {
        Self { redis }
    }

    pub async fn handle_connection(&self, stream: TcpStream) -> Result<()> {
        let mut stream = stream;

        loop {
            let mut buffer = BytesMut::with_capacity(1024);
            let read_size = stream.read_buf(&mut buffer).await?;
            if read_size == 0 {
                println!("Empty message, shutting down connection.");
                stream.shutdown().await?;
                return Ok(());
            }
            let now = time::Instant::now();
            let frame = std::str::from_utf8(&buffer)?;
            let commands = scanner::scan(frame);

            if commands.is_empty() {
                let resp = &Response::error("No supported command found");
                stream.write_all(resp.into()).await?;
                continue;
            }

            let responses = commands.iter().filter_map(|c| self.redis.handle(c, now));
            for response in responses {
                stream.write_all((&response).into()).await?;
            }
        }
    }
}
