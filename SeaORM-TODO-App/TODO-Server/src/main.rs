mod fruits_table;
mod suppliers_table;
mod todos_table;

pub use fruits_table::prelude::*;
pub use suppliers_table::prelude::*;
pub use todos_table::prelude::*;

use sea_orm::{
    sea_query::{Alias, ColumnDef, ForeignKey, ForeignKeyAction, Table},
    ConnectionTrait, Database, DbBackend,
};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    //Define the database backend
    let db_postgres = DbBackend::Postgres;

    // Read the database environment from the `.env` file
    let env_database_url = include_str!("../.env").trim();
    // Split the env url
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    // Get item with the format `database_backend://username:password@localhost/database`
    let database_url = split_url[1];

    // Perform a database connection
    let db = Database::connect(database_url).await?;

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
