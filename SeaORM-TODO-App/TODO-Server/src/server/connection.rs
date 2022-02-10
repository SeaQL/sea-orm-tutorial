// Import the necessary async versions of TcpStream and TcpListener
use crate::Command;
use async_std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream},
    prelude::*,
};

// function is called to create a new server on port 8080 localhost
pub async fn start_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on 127.0.0.1:8080");
    let mut incoming = listener.incoming();

    // Listen for an incoming stream
    while let Some(stream) = listener.incoming().next().await {
        let mut stream = stream?;

        let mut buf = [0u8; 4096];
        match stream.read(&mut buf).await {
            Ok(_) => {
                // Serialize the incoming stream
                let command: Command = bincode::deserialize(&buf)?;
                println!("{:?}", command);
            }
            Err(e) => println!("Unable to read stream: {}", e),
        }
    }

    Ok(())
}
