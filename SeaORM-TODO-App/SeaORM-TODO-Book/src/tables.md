# Creating Tables

First, create a database config for the `sea_orm::DatabaseConnection` to use to connect and authenticate to the PostgreSQL server.

```rust,no_run,noplayground
+ use async_std::sync::Arc;
+ use sea_orm::{Database, DbBackend};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    //Define the database backend
    + let db_postgres = DbBackend::Postgres;

    // Read the database environment from the `.env` file
    + let env_database_url = include_str!("../.env").trim();
    // Split the env url
    + let split_url: Vec<&str> = env_database_url.split("=").collect();
    // Get item with the format `database_backend://username:password@localhost/database`
    + let database_url = split_url[1];

    // Perform a database connection
    + let db = Database::connect(database_url).await?;
    + let db = Arc::new(db);

    Ok(())
}
```

The `include_str!("../.env").trim();` reads the `.env` file and loads it's content at compile time. This content is the PostgreSQL database configuration which we later split using `split("=")` and discard the `DATABASE_URL=` part since it's only needed by `sea-orm-cli` and not ` Database::connect()` which only accepts `database_backend://username:password@localhost/database`.

Calling the `Database::connect()` on the parsed URL creates a `DatabaseConnection` that will perform all CRUD operations. This connection is kept behind an `async_std::sync::Arc` for thread safety when we `spawn` async tasks.



Add the code to create the three tables, `todos`, `fruits` and `suppliers`.

**FILE**:***src/main.rs***

```rust,no_run,noplayground
  use async_std::sync::Arc;
- use sea_orm::{Database, DbBackend};
+ use sea_orm::{
+     sea_query::{Alias, ColumnDef, ForeignKey, ForeignKeyAction, Table},
+     ConnectionTrait, Database, DbBackend,
+ };

#[async_std::main]
async fn main() -> anyhow::Result<()> {

// --- code snippet ---

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
        .col(
            ColumnDef::new(Alias::new("date_time"))
                .timestamp()
                .not_null(),
        )
        .to_owned();

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

    // Create the `suppliers` table
    let suppliers_table = Table::create()
        .table(Alias::new("suppliers"))
        .if_not_exists()
        .col(
            ColumnDef::new(Alias::new("suppliers_id"))
                .integer()
                .auto_increment()
                .primary_key()
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("suppliers_name"))
                .string()
                .unique_key()
                .not_null(),
        )
        .col(ColumnDef::new(Alias::new("fruit_id")).integer().not_null())
        .col(
            ColumnDef::new(Alias::new("date_time"))
                .timestamp()
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_supplier_fruits")
                .from(Alias::new("suppliers"), Alias::new("fruit_id"))
                .to(Alias::new("fruits"), Alias::new("fruit_id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned();

    // Executing the SQL query to create the `suppliers` table in PostgreSQL
    let create_table_op = db.execute(db_postgres.build(&suppliers_table)).await;
    // Print the result in a user friendly way
    println!(
        "`CREATE TABLE suppliers` {:?}",
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

$ sea-orm-cli generate entity -o src/suppliers_list_table -t suppliers #The suppliers table
```

This generates new directories

```sh
SeaORM-TODO-App
	|-- Cargo.toml
	|-- .env
	|-- src
+ 	|-- suppliers_list_table
+ 		|-- mod.rs
+ 		|-- prelude.rs
+ 		|-- suppliers.rs
+ 	|-- fruits_list_table
+ 		|-- mod.rs
+ 		|-- prelude.rs
+ 		|-- fruits.rs
+ 	|-- todo_list_table
+ 		|-- mod.rs
+ 		|-- prelude.rs
+ 		|-- todos.rs
```

Modify the `src/suppliers_list_table/prelude.rs` and import the types using friendly names.

```rust,no_rust,noplayground
- pub use super::suppliers::Entity as Suppliers;

+ pub use super::suppliers::{
+     ActiveModel as SuppliersActiveModel, Column as SuppliersColumn, Entity as Suppliers,
+     Model as SuppliersModel, PrimaryKey as SuppliersPrimaryKey, Relation as SuppliersRelation,
+ };
```

Do the same to the `src/fruits_table/prelude.rs`

```rust,no_run,noplayground
- pub use super::fruits::Entity as Fruits;

+ pub use super::fruits::{
+     ActiveModel as FruitsActiveModel, Column as FruitsColumn, Entity as Fruits,
+     Model as FruitsModel, PrimaryKey as FruitsPrimaryKey, Relation as FruitsRelation,
+ };
```

Do the same ot the `src/todos_table/prelude.rs`

```rust,no_run,noplayground
//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0
- pub use super::todos::Entity as Todos;

+ pub use super::todos::{
+     ActiveModel as TodosActiveModel, Column as TodosColumn, Entity as Todos, Model as TodosModel,
+     PrimaryKey as TodosPrimaryKey, Relation as TodosRelation,
+ };
```

Modify the `Model` and `Relation` part of `Suppliers` Entity to  import `Fruits` entity properly

```rust,no_run,noplayground
//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "suppliers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub suppliers_id: i32,
    #[sea_orm(unique)]
    pub suppliers_name: String,
    pub fruit_id: i32,
    pub date_time: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
-		belongs_to = "super::fruits::Entity",
+		belongs_to = "crate::Fruits",
        from = "Column::FruitId",
-		to = "super::fruits::Column::FruitId",
+		to = "crate::FruitsColumn::FruitId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Fruits,
}

- impl Related<super::fruits::Entity> for Entity {
+ impl Related<crate::Fruits> for Entity {
    fn to() -> RelationDef {
        Relation::Fruits.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

```

Import these modules into `src/main.rs`

```rust,no_run,noplayground
// --- code snippet ---
+ mod fruits_list_table;
+ mod suppliers_list_table;
+ mod todo_list_table;

+ pub use fruits_list_table::prelude::*;
+ pub use suppliers_list_table::prelude::*;
+ pub use todo_list_table::prelude::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // --- code snippet ---
    
    Ok(())
 }
```

Next, populate the `fruits` and `suppliers` tables with data. First, add `chrono` crate to `Cargo.toml` file.

```sh
$ cargo add chrono
```

Then,  Create a new file `src/insert_values.rs` and add the following code:

```rust,no_run,noplayground
use crate::{Fruits, FruitsActiveModel, Suppliers, SuppliersActiveModel};
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

// Insert suppliers in the `suppliers` table
pub async fn insert_fruits(db: &DatabaseConnection) -> anyhow::Result<()> {
    // Get current system time
    let now = Utc::now();

    let apple = FruitsActiveModel {
        fruit_name: Set("Apple".to_owned()),
        date_time: Set(now),
        ..Default::default()
    };

    let orange = FruitsActiveModel {
        fruit_name: Set("Orange".to_owned()),
        date_time: Set(now),
        ..Default::default()
    };

    let mango = FruitsActiveModel {
        fruit_name: Set("Mango".to_owned()),
        date_time: Set(now),
        ..Default::default()
    };

    let pineapple = FruitsActiveModel {
        fruit_name: Set("Pineapple".to_owned()),
        date_time: Set(now),
        ..Default::default()
    };

    let fruit_insert_operation = Fruits::insert_many(vec![apple, orange, mango, pineapple])
        .exec(db)
        .await;

    println!("INSERTED FRUITS: {:?}", fruit_insert_operation?);

    Ok(())
}

// Insert suppliers in the `suppliers` table
pub async fn insert_suppliers(db: &DatabaseConnection) -> anyhow::Result<()> {
    // Get current system time
    let now = Utc::now();

    let john_doe = SuppliersActiveModel {
        suppliers_name: Set("John Doe".to_owned()),
        fruit_id: Set(1),
        date_time: Set(now),
        ..Default::default()
    };

    let jane_doe = SuppliersActiveModel {
        suppliers_name: Set("Jane Doe".to_owned()),
        fruit_id: Set(2),
        date_time: Set(now),
        ..Default::default()
    };

    let doe_junior = SuppliersActiveModel {
        suppliers_name: Set("Doe Junior".to_owned()),
        fruit_id: Set(3),
        date_time: Set(now),
        ..Default::default()
    };

    let doe_senior = SuppliersActiveModel {
        suppliers_name: Set("Doe Senior".to_owned()),
        fruit_id: Set(4),
        date_time: Set(now),
        ..Default::default()
    };

    let suppliers_insert_operation =
        Suppliers::insert_many(vec![john_doe, jane_doe, doe_senior, doe_junior])
            .exec(db)
            .await;

    println!("INSERTED SUPPLIERS: {:?}", suppliers_insert_operation?);

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
+    insert_suppliers(&db).await?;
    
     Ok(())
}
```

