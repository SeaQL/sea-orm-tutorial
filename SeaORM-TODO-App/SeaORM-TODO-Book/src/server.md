# Building Server Connections and Responses

Create a new director in the `src` folder called `server`.

Inside the `server` directory create `mod.rs`, `tcp_api.rs` and `connection.rs` 

```sh
SeaORM-TODO-App/src
				|-- src/fruits_table
                |-- src/insert_values.rs
                |-- src/main.rs
                |-- src/suppliers_table
                |-- src/todos_table 
                |-- src/server
+                	|-- mod.rs
+                	|-- tcp_api.rs
+                	|-- connection.rs
```

Import the new submodules to `src/server/mod.rs` file

```rust,no_run,no_playground
mod tcp_api;
mod connection;

pub use tcp_api::*;
pub use connection::*;
```

Then register the module to the `src.main.rs` file

```
// -- code snippet --

mod server;
pub use server::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
	// -- code snippet --
	
	Ok(())
}
```



#### The TCP API

Create the commands that the tcp api will handle. In the `src/server/tcp_api.rs` add:

```rust,no_run,noplayground
use serde::{Serialize, Deserialize};

// The commands to use to perform CRUD operations on PostgreSQL
#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Store { username: String, todo_list: String },
    UpdateTodoList { username: String, todo_list: String },
    Get(String),
    CreateUser(String),
    ListFruits,
}
```

`Command::Store { username: String, todo_list: String }` will handle an insert operation, inserting the `todo_list` in the row with the column labeled by `username`.

`Command::UpdateTodoList { username: String, todo_list: String }` will handle an update operation, inserting the `todo_list` in the row with the column labeled by `username`.

`Command::Get(String)` will fetch the `todo_list` from the column `username` with the username in the `String` field.

`Command::CreateUser(String)` will create a new row with the `String` field being inserted in the `username` column.

`Command::ListFruits` will fetch all the fruits in the `fruits` table.

The `Command` enum will be deserialized by `bincode` crate. Add the `bincode` and `serde` crates to `Cargo.toml` file

```sh
$ cargo add bincode

$ cargo add serde --features derive
```

Add error handling capabilities incase the wrong command is invoked

`File: src/server/tcp_api.rs`

```rust,no_rust,noplayground
// -- code snippet --

#[derive(Debug)]
pub enum ServerErrors {
    InvalidCommand,
    ModelNotFound,
}

impl Error for ServerErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ServerErrors::InvalidCommand => Some(&crate::ServerErrors::InvalidCommand),
            ServerErrors::ModelNotFound => Some(&crate::ServerErrors::ModelNotFound),
        }
    }
}

impl fmt::Display for ServerErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                ServerErrors::InvalidCommand => "Invalid command provided",
                ServerErrors::ModelNotFound => "The result of the query is `None`",
            }
        )
    }
}
```

The `ServerErrors::InvalidCommand` is returned when the method called on a `Command` is invalid while the `ServerErrors::ModelNotFound` is returned when a `Model` is not found in the database.

Then implement the methods for the `Command` enum that will handle database operations

`File: src/server/tcp_api.rs`

```rust,no_run,noplayground
// -- code snippet --
+ use crate::{Fruits, Suppliers, Todos, TodosActiveModel, TodosColumn, TodosModel};
+ use sea_orm::{
+     ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set,
+ };
  use serde::{Deserialize, Serialize};
+ use std::{error::Error, fmt};

// The commands to use to perform CRUD operations on PostgreSQL
#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Store { username: String, todo_list: String },
    UpdateTodoList { username: String, todo_list: String },
    Get(String),
    CreateUser(String),
    ListFruits,
}


```

Implement methods to handle the commands from the api

`File: src/server/tcp_api.rs`

```rust,no_run,noplayground
// -- code snippet --
// The commands to use to perform CRUD operations on PostgreSQL
#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Store { username: String, todo_list: String },
    UpdateTodoList { username: String, todo_list: String },
    Get(String),
    CreateUser(String),
    ListFruits,
}

impl Command {
    pub async fn get_fruits(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        let fruit_models = Fruits::find().all(db).await?;
        let fruits = fruit_models
            .iter()
            .map(|fruit_model| fruit_model.fruit_name.clone())
            .collect::<Vec<String>>();

        Ok(bincode::serialize(&fruits)?)
    }

    pub async fn store(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::Store {
                username,
                todo_list,
            } => {
                let todo_user = TodosActiveModel {
                    username: Set(username.to_owned()),
                    todo_list: Set(Some(todo_list.to_owned())),
                    ..Default::default()
                };
                Todos::insert(todo_user).exec(db).await?;

                Ok(bincode::serialize("INSERTED")?)
            }
            _ => Err(anyhow::Error::new(ServerErrors::InvalidCommand)),
        }
    }

    pub async fn create_new_user(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::CreateUser(username) => {
                let todo_user = TodosActiveModel {
                    username: Set(username.to_owned()),
                    ..Default::default()
                };
                Todos::insert(todo_user).exec(db).await?;

                Ok(bincode::serialize(&format!("CREATED_USER `{}`", username))?)
            }
            _ => Err(anyhow::Error::new(ServerErrors::InvalidCommand)),
        }
    }

    pub async fn get_user_todo(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::Get(user) => {
                let get_todo = Todos::find()
                    .filter(TodosColumn::Username.contains(user))
                    .one(db)
                    .await?;

                if let Some(found_todo) = get_todo {
                    Ok(bincode::serialize(&found_todo.todo_list)?)
                } else {
                    Ok(bincode::serialize(&Some("USER_NOT_FOUND"))?)
                }
            }
            _ => Err(anyhow::Error::new(ServerErrors::InvalidCommand)),
        }
    }

    pub async fn update_todo_list(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::UpdateTodoList {
                username,
                todo_list,
            } => {
                let found_todo: Option<TodosModel> = Todos::find()
                    .filter(TodosColumn::Username.contains(username))
                    .one(db)
                    .await?;

                match found_todo {
                    Some(todo_model) => {
                        let mut todo_model: TodosActiveModel = todo_model.into();
                        todo_model.todo_list = Set(Some(todo_list.to_owned()));
                        todo_model.update(db).await?;
                    }
                    None => return Err(anyhow::Error::new(ServerErrors::ModelNotFound)),
                };

                Ok(bincode::serialize("UPDATED_TODO")?)
            }
            _ => Err(anyhow::Error::new(ServerErrors::InvalidCommand)),
        }
    }
}

```

The `get_fruits()` method  handles the `Command::ListFruits`  command and it is responsible for fetching the list of fruits in the database.

The `store()` method handles the `Command:: Store {..}` command and it inserts the field `todo_list` in the `username` column corresponding to the `username` field.

The `create_new_user()` method handles `Command::CreateUser(..)` command, it creates a new user by inserting the `String` field data to the `username` column and an empty entry in the `todo_list` column.

The `get_user_todo()` method handles `Command::Get(..)` command. It is used mostly to check if the user in the `String` field exists in the `username` column.

The `update_todo_list()` method handles the `Command:: UpdateTodoList {..}` command and it updates the field `todo_list` in the `username` column corresponding to the `username` field.

#### The TCP API handler

The `TcpStream` will need to be handled. The `src/server/connection.rs` file contains the code for this.

`File: src/server/connection.rs`

```rust,no_run.noplayground
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
        Command::ListSuppliers => command.get_suppliers(db).await,
        Command::Store { .. } => command.store(db).await,
        Command::UpdateTodoList { .. } => command.update_todo_list(db).await,
        Command::DeleteUser(..) => command.delete_user(db).await,
    };

    match db_op {
        Ok(value) => Ok(value),
        Err(error) => Ok(bincode::serialize(&error.to_string())?),
    }
}

```

Here, the `BUFFER_DATA_CAPACITY` caps the TODO list data at `1MiB` and limits the buffer capacity for the TCP stream using `BUFFER_CAPACITY` capped at `64KiB`.

The `start_server()` function  creates a `TcpListener` at port `8080` localhost IP `127.0.0.1`. It  accepts a database connection inside an `Arc<DatabaseConnection>` for thread safety when we spawn a task to handle the stream. Each `TcpStream` is handled asynchronously using a `async::task::spawn()` method. 

`stream.read(&mut buffer).await?;` reads the stream. The while loop loops until the stream returns a `0_usize` indicating the connection has been closed by the peer and if data has been received, it checks if the data has exceeded the 

`BUFFER_DATA_CAPACITY` of `1MiB`, if not it decodes the buffer using `bincode` and passes the data to the `process_database_op()` function which matches the deserialized `Command` and calls the appropriate method which in turn performs the database operation, encodes the result of the database operation and writes it back to the peer using the `handle_response()` function.

#### Start the server

Lastly, inside the `src/main.rs` file, start the server

`File: src/main.rs`

```rust,no_run,noplayground
use async_std::sync::Arc;
use sea_orm::{
    sea_query::{Alias, ColumnDef, ForeignKey, ForeignKeyAction, Table},
    ConnectionTrait, Database, DbBackend,
};

// -- code snippet --

#[async_std::main]
async fn main() -> anyhow::Result<()> {

	// -- code snippet --
	
	
    insert_fruits(&db).await?;
    insert_suppliers(&db).await?;

+	start_server(db).await?;

    Ok(())
}

```

Run the program using `cargo run`. It print the following to the terminal

```sh
$ Running `/media/su43/IGIED-01/Rust-Projects/SeaQL/SeaORM-TODO-App/target/debug/todo-server`
`CREATE TABLE fruits` "Operation Successful"
`CREATE TABLE suppliers` "Operation Successful"
`CREATE TABLE todos` "Operation Successful"
INSERTED FRUITS: InsertResult { last_insert_id: 1 }
INSERTED SUPPLIERS: InsertResult { last_insert_id: 1 }
Listening on 127.0.0.1:8080
```

The server is now listening on `127.0.0.1:8080` for incoming `TcpStream`s.

Next, we build the client.
