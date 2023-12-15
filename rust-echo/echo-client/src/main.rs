use std::error::Error;
use tokio::net::TcpListener;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _tcp_listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Hello, world!");
    Ok(())
}
