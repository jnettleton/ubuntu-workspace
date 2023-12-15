use std::error::Error;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut tcp_socket, _) = tcp_listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                // read data from socket
                let n = match tcp_socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return, //socket closed
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket. error = {:?}", e);
                        return;
                    }
                };

                // write data back
                if let Err(e) = tcp_socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket.  error = {:?}", e);
                    return;
                }
            }
        });
    }
}
