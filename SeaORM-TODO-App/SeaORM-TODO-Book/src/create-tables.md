# Create Tables using SeaORM

In this tutorial, we will use builder pattern provided by `sea-orm` crate to create a table and insert it into the database.

#### Create the `fruits` table

```rust,noplayground,no_run
+ use sea_orm::{
+     sea_query::{Alias, ColumnDef, Table},
+     ConnectionTrait, Database, DbBackend,
+ };

#[async_std::main]
async fn main() -> anyhow::Result<()> {
+     let db_postgres = DbBackend::Postgres;
    
+     // Read the database environment from the `.env` file
+     let env_database_url = include_str!("../.env").trim();
+     // Split the env url
+     let split_url: Vec<&str> = env_database_url.split("=").collect();
+     // Get item with the format `database_backend://username:password@localhost/database`
+     let database_url = split_url[1];

+     let db = Database::connect(database_url).await?;

+     let fruits_table = Table::create()
+         .table(Alias::new("fruits"))
+         .if_not_exists()
+         .col(
+             ColumnDef::new(Alias::new("fruit_id"))
+                 .integer()
+                 .auto_increment()
+                 .primary_key()
+                 .not_null(),
+          )
+         .col(
+             ColumnDef::new(Alias::new("fruit_name"))
+                 .string()
+                 .unique_key()
+                 .not_null(),
+         )
+         .col(
+             ColumnDef::new(Alias::new("date_time"))
+                 .timestamp()
+                 .not_null(),
+         )
+         .to_owned();


+     let create_table_op = db.execute(db_postgres.build(&fruits_table)).await;
+     println!(
+         "`CREATE TABLE fruits` {:?}",
+         match create_table_op {
+             Ok(_) => "Operation Successful".to_owned(),
+             Err(e) => format!("Unsuccessful - Error {:?}", e),
+         }
+     );

+     Ok(())
+ }

```



To build an SQL statement with SeaORM, we need to define the database backend, for our case, it's PostgreSQL  and is defined by `DbBackend::Postgres` which we assign to the variable `db_postgres`. The `Table::create()` instantiates a `TableCreateStatement` which contains methods like `.col` and `.table`. We use the `Alias::new()` to define the table name called `fruits` and pass it to the `.table()` method. `if_not_exists()` method ensures that the database driver dosen't return an error if the table already exists.

The `ColumnDef` is used to define a table column and we use the `Alias::new()` to name all columns. Subsequent methods mirror SQL data types and values like `.integer()` for the SQL `INTEGER` or `SERIAL` data type and `.autoincrement()` method for SQL `AUTO_INCREMENT` . Refer to the latest docs for other methods [sea_orm::entity::prelude::ColumnDef](https://docs.rs/sea-orm/latest/sea_orm/entity/prelude/struct.ColumnDef.html). Passing the values of the `ColumnDef` allows passing this to the `.col()` method for `Table::create()` thereby creating the necessary colums `fruit_id`, `fruit_name` and `date_time`.

To execute the contents of the `Table::create()` methods and create the table in the database, we first get the database configuration that we defined in the `.env` file. For efficiency, we load the contents of the configuration at compile time using `include_str!("../.env").trim()` then we split the contents using `env_database_url.split("=").collect()` and then get the path using `split_url[1]`. This step is necessary to perform since `sea-orm-cli` and the `Databas::connect()` method expect different formats of the same configuration. We will use `sea-orm-cli` in a bit in order to autogenerate the database code into Rust modules.

Lastly, the `db.execute(db_postgres.build(&fruits_table))` method executes the SQL query to create the `fruits_table` in the PostgreSQL database.

Do the same to create the `suppliers` table and `todos` table.



#### Creating Suppliers table

```rust,noplayground,no_run
use sea_orm::{
-	sea_query::{Alias, ColumnDef, Table},
+	sea_query::{Alias, ColumnDef, ForeignKey, ForeignKeyAction, Table},
	//-- snippet --

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    //-- snippet --


    // Create the suppliers table
+     let suppliers_table = Table::create()
+         .table(Alias::new("suppliers"))
+         .if_not_exists()
+         .col(
+             ColumnDef::new(Alias::new("suppliers_id"))
+                 .integer()
+                 .auto_increment()
+                 .primary_key()
+                 .not_null(),
+         )
+         .col(
+             ColumnDef::new(Alias::new("suppliers_name"))
+                 .string()
+                 .unique_key()
+                 .not_null(),
+         )
+         .col(ColumnDef::new(Alias::new("fruit_id")).integer().not_null())
+         .col(
+             ColumnDef::new(Alias::new("date_time"))
+                 .timestamp()
+                 .not_null(),
+         )
+         .foreign_key(
+             ForeignKey::create()
+                 .name("FK_supplier_fruits")
+                 .from(Alias::new("suppliers"), Alias::new("fruit_id"))
+                 .to(Alias::new("fruits"), Alias::new("fruit_id"))
+                 .on_delete(ForeignKeyAction::Cascade)
+                 .on_update(ForeignKeyAction::Cascade),
+         )
+         .to_owned();


    // Executing the SQL query to create the `suppliers` table in PostgreSQL
+     let create_table_op = db.execute(db_postgres.build(&suppliers_table)).await;
    // Print the result in a user friendly way
+     println!(
+         "`CREATE TABLE suppliers` {:?}",
+         match create_table_op {
+             Ok(_) => "Operation Successful".to_owned(),
+             Err(e) => format!("Unsuccessful - Error {:?}", e),
+         }
+     );
    
    Ok(())
 }

```

The `suppliers` table references the `fruits` table hence the need to add a `FOREIGN KEY` contraint using `.foreign_key()` method with method argument `ForeignKeyStatement` created by the methods of `ForeignKey::create()` method.



#### Create `todos` table

The last table is the TODOs table to handle remote storage of user todo lists.

The structure of the of the table includes JSON formated Map of todos.

```rust,noplayground,no_run
use sea_orm::{
	//-- snippet --

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    //-- snippet --

    // Create the `todos` table
+     let todos_table = Table::create()
+         .table(Alias::new("todos"))
+         .if_not_exists()
+         .col(
+             ColumnDef::new(Alias::new("todo_id"))
+                 .integer()
+                 .auto_increment()
+                 .primary_key()
+                 .not_null(),
+         )
+         .col(
+             ColumnDef::new(Alias::new("username"))
+                 .string()
+                 .unique_key()
+                 .not_null(),
+         )
+         .col(ColumnDef::new(Alias::new("todo_list")).string())
+         .to_owned();

    // Executing the SQL query to create the `todos` table in PostgreSQL
+     let create_table_op = db.execute(db_postgres.build(&todos_table)).await;
    // Print the result in a user friendly way
+     println!(
+         "`CREATE TABLE todos` {:?}",
+         match create_table_op {
+             Ok(_) => "Operation Successful".to_owned(),
+             Err(e) => format!("Unsuccessful - Error {:?}", e),
+         }
+     );
    
    Ok(())
 }
```



Next, we autogenerate the code for the `Model`, `Entity`, `ActiveModel` ... etc . This code will perform CRUD operations on the database using SeaORM. Execute `sea-orm-cli` commands

1. Generate the `fruits` table code

   ```sh
   $ sea-orm-cli generate entity -o src/fruits_table -t fruits
   ```

2. Generate the `suppliers` table code

   ```sh
   $ sea-orm-cli generate entity -o src/suppliers_table -t suppliers
   ```

3. Generate the `todos` table code

   ```sh
   $ sea-orm-cli generate entity -o src/todos_table -t todos
   ```

This will create new code in the `src/` directory 

```rust,noplayground,no_run
+ 	src/fruits_table
	src/main.rs
+	src/suppliers_table
+ 	src/todos_table 
```

Rename the autogenerated code prelude to user friendly names for the `Model`, `Entity`, ..etc.

File: `src/fruits_table/prelude.rs`

```rust,noplayground,no_run
//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

- 	pub use super::fruits::Entity as Fruits;

+   pub use super::fruits::{
+       ActiveModel as FruitsActiveModel, Column as FruitsColumn, Entity as Fruits,
+       Model as FruitsModel, PrimaryKey as FruitsPrimaryKey, Relation as FruitsRelation,
+   };

```



File: `src/suppliers_table/prelude.rs`

```rust,noplayground,no_run
//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

- 	pub use super::suppliers::Entity as Suppliers;
	
+   pub use super::suppliers::{
+       ActiveModel as SuppliersActiveModel, Column as SuppliersColumn, Entity as Suppliers,
+       Model as SuppliersModel, PrimaryKey as SuppliersPrimaryKey, Relation as SuppliersRelation,
+   };
```



File: `src/todos_table/prelude.rs`

```rust,noplayground,no_run
//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

- 	pub use super::todos::Entity as Todos;

+	pub use super::todos::{
+	    ActiveModel as TodosActiveModel, Column as TodosColumn, Entity as Todos, Model as TodosModel,
+	    PrimaryKey as TodosPrimaryKey, Relation as TodosRelation,
+	};

```



Import these modules into the main file

File: `src/main.rs`

```rust,noplayground,no_run
+ mod fruits_table;
+ mod suppliers_table;
+ mod todos_table;

+ pub use fruits_table::prelude::*;
+ pub use suppliers_table::prelude::*;
+ pub use todos_table::prelude::*;

	//-- snippet --

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    //-- snippet --
    
    Ok(())
}
```



Lastly, modify the `src/suppliers_table/suppliers.rs` to import the `fruits_table` modules properly

`File: src/suppliers_table/suppliers.rs`

```rust,noplayground,no_run
//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "suppliers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub suppliers_id: i32,
    #[sea_orm(unique)]
    pub suppliers_name: String,
    pub fruit_id: i32,
    pub date_time: DateTime,
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



In the next section, we will create the TCP server and the commands for performing CRUD operations
