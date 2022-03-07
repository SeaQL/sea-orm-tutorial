use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use sea_orm::{
    sea_query::{Alias, ColumnDef, Table},
    ConnectionTrait, Database, DatabaseConnection, DbBackend,
};
use std::net::SocketAddr;

mod fruits_list_table;
mod insert_values;
mod routing;
mod todo_list_table;

pub use fruits_list_table::prelude::*;
pub use insert_values::*;
pub use routing::*;
pub use todo_list_table::prelude::*;

static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    insert_fruits(&db).await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/fruits", get(get_fruits))
        .route("/store", post(store_todo))
        .route("/update_todo", post(update_todo));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
