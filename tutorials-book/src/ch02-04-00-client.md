# Building The HTTP Client

This chapter focuses on creating the HTTP client. Switch to the `client` directory in the workspace.

#### Configuration

Add the necessary dependencies to create the client.

```sh
$ cargo add tokio --features full

$ cargo add anyhow

$ cargo add serde --features derive

$ cargo add serde_json

$ cargo add minreq

$ cargo add dotenv

$ cargo add json
```

`serde_json` crate will deserialize the TODO list data structure that contains queued and completed TODOs into a JSON string for remote storage in the PostgreSQL database. The `json` crate will serialize JSON data to be sent over HTTP using the `minreq` crate.

The todo client will also store local cache, simulating a real world setup especially for a desktop or mobile client. SQLite will be the preferred database for this tutorial due to it's popularity. A command line frontend  will be used to keep the tutorial simple and easy to port to other domains like mobile device frameworks, desktop clients.

Add `sea-orm` crate with the SQLite features enabled for the local persistent cache. The  runtime features `runtime-tokio-rustls` are used since the async library for this client is `tokio` crate.

```sh
$ cargo add sea-orm  --features "runtime-tokio-rustls sqlx-sqlite macros" --no-default-features
```

Modify the main function in  `src/main.rs` to use async-std

```rust,no_run,noplayground
- fn main() {
-     println!("Hello, world!");
- }

+ #[tokio::main]
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

Top perform local database operations, create a file `src/db_ops.rs` which will contain functions to perform database operations. 

To serialize and deserialize the SQLite cache for the in-memory database, the struct `TodoList` is used:

```rust,no_run,noplayground
//  The structure for a TodoList
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct TodoList {
    pub queued: Vec<MyTodosModel>,
    pub completed: Vec<MyTodosModel>,
}
```

This data structure holds the completed TODOs in the `completed` field and the incompleted TODOs in the `queued` field. Both of this fields hold a `Vec<MyTodosModel` which ensures that no database fetch requests are necessary to make modifications savingon I/O operations that would otherwise have to fetch the `Model` before converting the `Model` into an `ActiveModel` and doing modifications.

The function `create_todo_table()` when invoked will create a new `todo_list` table in the local SQLite database specified by the URL.

`File: client/src/db_ops.rs`

```rust,no_run,noplayground

use sea_orm::{
    sea_query::{Alias, ColumnDef, Table},
    ActiveModelTrait, ConnectionTrait, Database, DatabaseConnection, EntityTrait, Set,
};
use serde::{Serialize, Deserialize};

//  The structure for a TodoList
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct TodoList {
    pub queued: Vec<MyTodosModel>,
    pub completed: Vec<MyTodosModel>,
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

To use the `dotenv` crate to read the `DATABASE_URL` environment variable, add the following code to `src/main.rs`.

`File: client/src/main.rs`

```rust,no_run.noplayground
+ use dotenv::dotenv;
+ use sea_orm::Database;

// -- code snippet --
#[tokio::main]
async fn main() -> anyhow::Result<()>{

+   dotenv().ok();

    // Read the database environment from the `.env` file
+   let database_url = dotenv::var("DATABASE_URL")?;
+   let db = Database::connect(database_url).await?;
    Ok(())
}
```

Then import the `db_ops` module into `src/main.rs`	 and call both functions.

```rust,no_run,noplayground
// -- code snippet --

+ mod db_ops;
+ pub use db_ops::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // -- code snippet --

    // Read the database environment from the `.env` file
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;

+   create_todo_table(&db).await?;

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
