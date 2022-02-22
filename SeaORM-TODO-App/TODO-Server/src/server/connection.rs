// Import the necessary async versions of TcpStream and TcpListener
use crate::Command;
use async_std::{
    net::{Shutdown, SocketAddr, TcpListener, TcpStream},
    prelude::*,
    sync::Arc,
    task,
};

use sea_orm::DatabaseConnection;

const BUFFER_DATA_CAPACITY: usize = 1024 * 1024; // The todo list should not exceed 1MiB
const BUFFER_CAPACITY: usize = 64 * 1024; //64Kib

// function is called to create a new server on port 8080 localhost
pub async fn start_server(db: Arc<DatabaseConnection>) -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on 127.0.0.1:8080");

    while let Some(stream) = listener.incoming().next().await {
        let stream = stream?;
        let db = db.clone();

        task::spawn(async move {
            match process_stream(db, stream).await {
                Ok(addr) => {
                    println!("x → {addr:?} - DISCONNECTED")
                }
                Err(error) => {
                    eprintln!("{:?}", error);
                }
            }
        })
        .await;
    }

    Ok(())
}

async fn process_stream(
    db: Arc<DatabaseConnection>,
    mut stream: TcpStream,
) -> anyhow::Result<SocketAddr> {
    let peer = stream.peer_addr()?;
    println!("← → {peer:?} - CONNECTED");
    let mut buffer = [0u8; BUFFER_CAPACITY];
    let mut command_buffer: Vec<u8> = Vec::new();
    let bytes_read = stream.read(&mut buffer).await?;
    while bytes_read != 0 {
        if command_buffer.len() > BUFFER_DATA_CAPACITY {
            handle_response(&mut stream, b"BUFFER_CAPACITY_EXCEEDED_1MiB".to_vec()).await?;
        }

        // Check if the current stream is less than the buffer capacity, if so all data has been received
        if buffer[..bytes_read].len() < BUFFER_CAPACITY {
            // Ensure that the data is appended before being deserialized by bincode
            command_buffer.append(&mut buffer[..bytes_read].to_owned());
            let dbop_result = process_database_op(&db, &command_buffer).await?;
            handle_response(&mut stream, dbop_result).await?;
            break;
        }
        // Append data to buffer
        command_buffer.append(&mut buffer[..bytes_read].to_owned());
    }

    let peer = stream.peer_addr()?;
    //Shutdown the TCP address
    stream.shutdown(Shutdown::Both)?;
    // Terminate the stream if the client terminates the connection by sending 0 bytes
    return Ok(peer);
}

async fn handle_response(stream: &mut TcpStream, reponse_data: Vec<u8>) -> anyhow::Result<()> {
    stream.write_all(&reponse_data).await?;

    stream.flush().await?;

    Ok(())
}

async fn process_database_op(
    db: &DatabaseConnection,
    command_buffer: &[u8],
) -> anyhow::Result<Vec<u8>> {
    let command: Command = bincode::deserialize(command_buffer)?;

    let db_op = match command {
        Command::Get(..) => command.get_user_todo(db).await,
        Command::CreateUser(..) => command.create_new_user(db).await,
        Command::ListFruits => command.get_fruits(db).await,
        Command::Store { .. } => command.store(db).await,
        Command::UpdateTodoList { .. } => command.update_todo_list(db).await,
    };

    match db_op {
        Ok(value) => Ok(value),
        Err(error) => Ok(bincode::serialize(&error.to_string())?),
    }
}
