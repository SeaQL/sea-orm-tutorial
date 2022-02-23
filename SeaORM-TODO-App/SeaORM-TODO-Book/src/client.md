# Building The TODO TCP Client

This chapter focuses on creating the TCP client. Switch to the `TODO-Client` directory in the workspace.

#### Configuration

Add the necessary dependencies to create the client.

```sh
$ cargo add async-std --features attributes

$ cargo add anyhow

$ cargo add bincode

$ cargo add serde --features derive

$ cargo add serde_json
```

`bincode` crate will be used to prepare the bytes of  `Command` to send over the wire. `serde_json` crate will serialize the TODO list data structure that contains queued and completed TODOs into a JSON string for remote storage in the PostgreSQL database.

The TCP client will also store local cache, simulating a real world setup especially for a desktop or mobile client. SQLite will be the preferred database for this tutorial due to it's popularity. A command line frontend and a TCP stream will be used to keep the tutorial simple and easy to port to other domains like mobile device connection, desktop clients or HTTP clients if you wish to explore other domains.

Add `sea-orm` crate with the SQLite features enabled for the local persistent cache. The  runtime features `runtime-async-std-rustls` are used since the async library for this client is `async-std` crate.

```sh
$ cargo add sea-orm  --features "runtime-async-std-rustls sqlx-sqlite macros" --no-default-features
```

Modify the main function in  `src/main.rs` to use async-std

```rust,no_run,noplayground
- fn main() {
-     println!("Hello, world!");
- }

+ #[async_std::main]
+ async fn main() -> anyhow::Result<()>{
+     Ok(())
+ }
```

Next, create a `.env` file in the current directory. This will contain the database configuration.

`File: TODO-Client/.env`

```sh
DATABASE_URL=sqlite://my_todos.db
```

Here, the `sqlite` URL does not take a `username`, `password` and `IP` since SQLite does not have have a server, just the database name `my_todos.db`.

Create an empty SQLite database using the command:

```sh
$ sqlite3 my_todos.db "VACUUM;"
```

The `"VACUUM;"` part of the command will ensure the created database is not just held in memory but also persisted to the file system even though it is empty.

#### Local SQLite Database Operations

Create a file `src/db_ops.rs` which will contain functions to perform database operations.

```rust,no_run,noplayground
use async_std::{
    io::{ReadExt, WriteExt},
    net::TcpStream,
};
use sea_orm::{
    sea_query::{Alias, ColumnDef, Table},
    ActiveModelTrait, ConnectionTrait, Database, DatabaseConnection, EntityTrait, Set,
};

pub async fn database_config() -> Result<DatabaseConnection, sea_orm::DbErr> {
    // Read the database environment from the `.env` file
    let env_database_url = include_str!("../.env").trim();
    // Split the env url
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    // Get item with the format `database_backend://username:password@localhost/database`
    let database_url = split_url[1];

    Database::connect(database_url).await
}


pub async fn create_todo_table(db: &DatabaseConnection) -> anyhow::Result<()> {
    let database_backend = db.get_database_backend();
    // Create the `todos` table
    let todos_table = Table::create()
        .table(Alias::new("todo_list"))
        .if_not_exists()
        .col(
            ColumnDef::new(Alias::new("todo_id"))
                .integer()
                .primary_key()
                .not_null()
                .auto_increment(),
        )
        .col(
            ColumnDef::new(Alias::new("todo_name"))
                .string()
                .unique_key()
                .not_null(),
        )
        .col(ColumnDef::new(Alias::new("quantity")).string().not_null())
        .col(ColumnDef::new(Alias::new("status")).boolean().not_null())
        .to_owned();
    let create_table_op = db.execute(database_backend.build(&todos_table)).await;

    // Executing the SQL query to create the `todos` table in SQLite
    let create_table_op = db.execute(database_backend.build(&todos_table)).await;
    // Print the result in a user friendly way
    println!(
        "`CREATE TABLE todo_list` {:?}",
        match create_table_op {
            Ok(_) => "Operation Successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error {:?}", e),
        }
    );

    Ok(())
}

```

`database_config()` reads the `.env` file and parses the database URL, creates a database connection with the URL using `Database::connect()` and then returns a `DatabaseConnection`.

`create_todo_table()` when invoked will create a new `todo_list` table in the local SQLite database specified by the URL.

Import the `db_ops` module into `src/main.rs`	 and call both functions.

```rust,no_run,noplayground
+ mod db_ops;
+ pub use db_ops::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
+	let db = database_config().await?;
+	create_todo_table(&db).await?;
    
    Ok(())
}

```

Next is to auto-generate the `Model`, `ActiveModel` , `Entity`, etc... using the `sea-orm-cli` and pass in `--with-serde both` feature flag to auto-generate `serde::Serialize` and `serde::Deserialize` on the Entity.

```sh
$ sea-orm-cli generate entity -o src/todo_list_table -t todo_list --with-serde both
```

This will create a new directory `todo_list_table` in the `src/` directory. 

Open the `src/todo_list_table/prelude.rs` file and import the `Entity`, `Model` and `ActiveModel` using friendly names.

`File:src/todo_list_table/prelude.rs`

```rust,no_run,noplayground
//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

- pub use super::todo_list::Entity as TodoList;

+ pub use super::todo_list::{
+     ActiveModel as MyTodosActiveModel, Column as MyTodosColumn, Entity as MyTodos,
+     Model as MyTodosModel, PrimaryKey as MyTodosPrimaryKey, Relation as MyTodosRelation,
+ };

```

Import the modules to the `src/main.rs` file

```rust,no_run,noplayground
  mod db_ops;
+ mod todo_list_table;

  pub use db_ops::*;
+ pub use todo_list_table::prelude::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let db = database_config().await?;
    create_todo_table(&db).await?;

    Ok(())
}

```



#### Common Data Structures

To perform more database operations, create a `common.rs`  file in the `src` directory. This file will contain common data structures for use throughout database operations and TCP connections.

`File: src/common.rs`

```rust,no_run,noplayground
use crate::MyTodosModel;
use serde::{Deserialize, Serialize}; // The commands to use to perform CRUD operations on PostgreSQL

// The commands to use to perform CRUD operations on PostgreSQL
#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Store { username: String, todo_list: String },
    UpdateTodoList { username: String, todo_list: String },
    Get(String),
    CreateUser(String),
    ListFruits,
}

//  The structure for a TodoList
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct TodoList {
    pub queued: Vec<MyTodosModel>,
    pub completed: Vec<MyTodosModel>,
}

```

The enum `Command` mirrors the `Command` created in the previous chapter in the `TODO-Server/src/tcp_api.rs` file.

The `TodoList` struct contains the `Model`s `MyTodoModel` sorted either as `queued` which are TODOs not done or `completed` which are `TODO`s that are done.

Import this file to the `src/main.rs` file

```rust,no_run,noplayground
+ mod common;
  mod db_ops;
  mod todo_list_table;

+ pub use common::*;
  pub use db_ops::*;
  pub use todo_list_table::prelude::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let db = database_config().await?;
    create_todo_table(&db).await?;

    Ok(())
}

```
