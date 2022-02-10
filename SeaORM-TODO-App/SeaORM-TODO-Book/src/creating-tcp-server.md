# TCP Server

Since SeaORM is async first, we will take advantage of async-std async versions of standard library I/O for networking.

In the `src` folder, create a new directory called `server` and add files `mod.rs`, `tcp_api.rs` and `connection.rs`. The new folder structure should look like this:

```sh
SeaORM-TODO-App/TODO-Server/src
	|-- fruits_table
	|-- main.rs
+	|-- server
+			|-- connection.rs
+			|-- mod.rs
+			|-- tcp_api.rs
	|-- suppliers_table
	|-- todos_table

```

Import the submodules into the `mod.rs` file

```rust
mod connection;
mod tcp_api;

pub use connection::*;
pub use tcp_api::*;
```

Then import the module in the `main.rs` file

```rust
	//--- Snippet ---
	mod fruits_table;
+ 	mod server;

    #[async_std::main]
    async fn main() -> anyhow::Result<()> {
        //--- Snippet ---

        Ok(())
    }

```

The file `src/server/tcp_api.rs` will contain the API to perform database operations on the server side. Let's add a few commands to this file.

```rust
use crate::{Fruits, Suppliers, Todos, TodosActiveModel, TodosColumn, TodosModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

// The commands to use to perform CRUD operations on PostgreSQL
#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Store { username: String, todo_list: String },
    UpdateTodoList { username: String, todo_list: String },
    Get(String),
    ListFruits,
    ListSuppliers,
    DeleteUser(String),
}

impl Command {
    pub async fn get_fruits(db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        let fruit_models = Fruits::find().all(db).await?;
        let fruits = fruit_models
            .iter()
            .map(|fruit_model| fruit_model.fruit_name.clone())
            .collect::<String>();

        Ok(bincode::serialize(&fruits)?)
    }
    pub async fn get_suppliers(db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        let supplier_models = Suppliers::find().all(db).await?;
        let suppliers = supplier_models
            .iter()
            .map(|supplier_model| supplier_model.suppliers_name.clone())
            .collect::<String>();

        Ok(bincode::serialize(&suppliers)?)
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

    pub async fn get_user_todo(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::Get(user) => {
                let found_todo = Todos::find()
                    .filter(TodosColumn::Username.contains(user))
                    .all(db)
                    .await?;

                let mut todo_list = Vec::default();

                // Return API friendly `Vec` of `String`
                found_todo.iter().for_each(|todo_model| {
                    if let Some(todo_exists) = &todo_model.todo_list {
                        todo_list.push(todo_exists);
                    }
                });

                Ok(bincode::serialize(&todo_list)?)
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

                // Successful command return API friendly `UPDATED_TODO` as bytes
                Ok(bincode::serialize("UPDATED_TODO")?)
            }
            _ => Err(anyhow::Error::new(ServerErrors::InvalidCommand)),
        }
    }

    pub async fn delete_user(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::DeleteUser(user) => {
                let found_todo: Option<TodosModel> = Todos::find()
                    .filter(TodosColumn::Username.contains(user))
                    .one(db)
                    .await?;

                match found_todo {
                    Some(todo_model) => {
                        todo_model.delete(db).await?;
                    }
                    None => return Err(anyhow::Error::new(ServerErrors::ModelNotFound)),
                };
                
                // Successful command return API friendly `DELETED_USER` as bytes
                Ok(bincode::serialize("DELETED_USER")?)
            }
            _ => Err(anyhow::Error::new(ServerErrors::InvalidCommand)),
        }
    }
}


#[derive(Debug)]
pub enum ServerErrors {
    // Errors encountered while deserializing the buffer into a `Command`
    InvalidCommand,
    // The filter query returned an empty value after executing the query in the database
    ModelNotFound,
}

// Implement `std::error::Error` in order to pass it to `anyhow::Error` using `?`
impl Error for ServerErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&crate::ServerErrors::InvalidCommand)
    }
}

// Required for implementing `std::error::Error` 
impl fmt::Display for ServerErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                ServerErrors::InvalidCommand => "Invalid command provided",
                ModelNotFound => "The result of the query is `None`",
            }
        )
    }
}

```

1. The `Command::Store{ username: String, todo_list: String }` will store todo data `todo_list` for the user described in the `username` field.
1. `Command::UpdateTodoList{ username: String, todo_list: String }` updates the contents of the new todo list.
2. The `Command::Get(String)` will fetch the todo list of the user described in the `String`
3. `Command::ListFruits` will list all the fruits in the `fruits` table.
4. `Command::ListSuppliers` will list all the suppliers in the `suppliers` table
5. `Command::DeleteUser(String)` will delete a user described in the `String`.

The `get_fruits()`, `get_suppliers()`, `store()`, `get_user_todo()`, `update_todo_list` and `delete_user()` methods simply perform CRUD operations as illustrated in the previous `Simple_CRUD` tutorial. We serialize the outcome of these commands using `bincode::serialize()` method in order to efficiently transfer them through the `TcpStream`.

The file  `src/server/connection.rs` will contain the code to create the server.

```rust
// Import the necessary async versions of TcpStream and TcpListener
use async_std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream},
    prelude::*,
};
use crate::Command;

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
```

Most of this code is self explanatory since it is uniform across multiple programming languages with system programming capabilities. We create  a`TcpListener` and `bind()` it to port `8080` at localhost `127.0.0.1`. Next, we listen for a `TcpStream` and read any incoming stream into a buffer `buf` of `4096` bytes. SInce this is a simple application, the buffer size is acceptable. We then deserialize the bytes in buffer using bincode into a `Command` for further processing.

