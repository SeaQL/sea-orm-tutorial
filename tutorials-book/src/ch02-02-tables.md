# Creating Tables

First, create a database config for the `sea_orm::DatabaseConnection` to use to connect and authenticate to the PostgreSQL server. 

```rust,no_run,noplayground
+ use once_cell::sync::OnceCell;
+ use sea_orm::{DatabaseConnection, Database};
+ use dotenv::dotenv;

+ static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Read the database environment from the `.env` file
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    DATABASE_CONNECTION.set(db).unwrap();

    Ok(())
}
```

`dotenv::var()` is used to load the configuration `DATABASE_URL` as specified in the `.env` file. This is passed to the `Database::connect()` method in order to create a `sea_orm::DatabaseConnection` which executes queries in the database. The database connection is exported to the global scope using `once_cell` crate  as a static global variable 

`static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();`

This is later set to the `DatabaseConnection` using ` DATABASE_CONNECTION.set(db).unwrap();`. 

Add the code to create the tables, `todos` and `fruits`.

**FILE**:***src/main.rs***

```rust,no_run,noplayground
  use async_std::sync::Arc;
- use sea_orm::{Database, DatabaseConnection};
+ use sea_orm::{
+     sea_query::{Alias, ColumnDef, ForeignKey, ForeignKeyAction, Table},
+     ConnectionTrait, Database, DatabaseConnection, DbBackend,
+ };

#[async_std::main]
async fn main() -> anyhow::Result<()> {

// --- code snippet ---

//Define the database backend
    let db_postgres = DbBackend::Postgres;

    dotenv().ok();

    // Read the database environment from the `.env` file
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    DATABASE_CONNECTION.set(db).unwrap();

    // Create the fruits table
    let fruits_table = Table::create()
        .table(Alias::new("fruits"))
        .if_not_exists()
        .col(
            ColumnDef::new(Alias::new("fruit_id"))
                .integer()
                .auto_increment()
                .primary_key()
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("fruit_name"))
                .string()
                .unique_key()
                .not_null(),
        )
        .to_owned();

    let db = DATABASE_CONNECTION.get().unwrap();

    // Executing the SQL query to create the `fruits_table` in PostgreSQL
    let create_table_op = db.execute(db_postgres.build(&fruits_table)).await;
    // Print the result in a user friendly way
    println!(
        "`CREATE TABLE fruits` {:?}",
        match create_table_op {
            Ok(_) => "Operation Successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error {:?}", e),
        }
    );

    // Create the `todos` table
    let todos_table = Table::create()
        .table(Alias::new("todos"))
        .if_not_exists()
        .col(
            ColumnDef::new(Alias::new("todo_id"))
                .integer()
                .auto_increment()
                .primary_key()
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("username"))
                .string()
                .unique_key()
                .not_null(),
        )
        .col(ColumnDef::new(Alias::new("todo_list")).string())
        .to_owned();

    // Executing the SQL query to create the `todos` table in PostgreSQL
    let create_table_op = db.execute(db_postgres.build(&todos_table)).await;
    // Print the result in a user friendly way
    println!(
        "`CREATE TABLE todos` {:?}",
        match create_table_op {
            Ok(_) => "Operation Successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error {:?}", e),
        }
    );
    
	Ok(())
}
```

The previous tutorial gave an introduction on creating tables. `Table::create()` is the method to do this. Then the `db.execute()` method performs the database operation for table creation.

Next, use `sea-orm-cli` to auto-generate the code for `Entity`, `Model`, `Relation` , etc...

```sh
$ sea-orm-cli generate entity -o src/todo_list_table -t todos #The todos table

$ sea-orm-cli generate entity -o src/fruits_list_table -t fruits #The fruits table
```

This generates new directories

```sh
SeaORM-TODO-App
	|-- Cargo.toml
	|-- .env
	|-- src
+ 	|-- fruits_list_table
+ 		|-- mod.rs
+ 		|-- prelude.rs
+ 		|-- fruits.rs
+ 	|-- todo_list_table
+ 		|-- mod.rs
+ 		|-- prelude.rs
+ 		|-- todos.rs
```

Modify the `src/fruits_list_table/prelude.rs` and import the types using friendly names.

```rust,no_run,noplayground
- pub use super::fruits::Entity as Fruits;

+ pub use super::fruits::{
+     ActiveModel as FruitsActiveModel, Column as FruitsColumn, Entity as Fruits,
+     Model as FruitsModel, PrimaryKey as FruitsPrimaryKey, Relation as FruitsRelation,
+ };
```

Do the same to the `src/todo_list_table/prelude.rs`

```rust,no_run,noplayground
//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0
- pub use super::todos::Entity as Todos;

+ pub use super::todos::{
+     ActiveModel as TodosActiveModel, Column as TodosColumn, Entity as Todos, Model as TodosModel,
+     PrimaryKey as TodosPrimaryKey, Relation as TodosRelation,
+ };
```

Import these modules into `src/main.rs`

```rust,no_run,noplayground
// --- code snippet ---
+ mod fruits_list_table;
+ mod todo_list_table;

+ pub use fruits_list_table::prelude::*;
+ pub use todo_list_table::prelude::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // --- code snippet ---
    
    Ok(())
 }
```

Next, populate the `fruits` table with a list of fruits.

Create a new file `src/insert_values.rs` and add the following code:

```rust,no_run,noplayground
use crate::{Fruits, FruitsActiveModel};
use sea_orm::{DatabaseConnection, EntityTrait, Set};

// Insert suppliers in the `suppliers` table
pub async fn insert_fruits(db: &DatabaseConnection) -> anyhow::Result<()> {
    let apple = FruitsActiveModel {
        fruit_name: Set("Apple".to_owned()),
        ..Default::default()
    };

    let orange = FruitsActiveModel {
        fruit_name: Set("Orange".to_owned()),
        ..Default::default()
    };

    let mango = FruitsActiveModel {
        fruit_name: Set("Mango".to_owned()),
        ..Default::default()
    };

    let pineapple = FruitsActiveModel {
        fruit_name: Set("Pineapple".to_owned()),
        ..Default::default()
    };

    let fruit_insert_operation = Fruits::insert_many(vec![apple, orange, mango, pineapple])
        .exec(db)
        .await;

    println!("INSERTED FRUITS: {:?}", fruit_insert_operation?);

    Ok(())
}

```

Here, `ActiveModel` is used to prepare the data for insertion into the database using `Entity::insert()` .

Import this module to the `src/main.rs` file and call these functions to perform insert operations

```rust,no_run,noplayground
// --- code snippet ---
+ mod insert_values;
+ pub use insert_values::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // --- code snippet ---
    
+	 insert_fruits(&db).await?;
    
     Ok(())
}
```

