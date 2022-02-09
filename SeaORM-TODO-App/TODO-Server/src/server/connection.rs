use async_std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream},
    prelude::*,
};
use serde::{Deserialize, Serialize};

pub async fn start_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on 127.0.0.1:8080");
    let mut incoming = listener.incoming();

    while let Some(stream) = listener.incoming().next().await {
        let mut stream = stream?;

        let mut buf = [0u8; 4096];
        match stream.read(&mut buf).await {
            Ok(_) => {
                let command: Command = bincode::deserialize(&buf)?;
                println!("{:?}", command);
            }
            Err(e) => println!("Unable to read stream: {}", e),
        }
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Store { username: String, todo_list: String },
    Get(String),
    ListFruits,
    ListSuppliers,
    DeleteTodo,
}
