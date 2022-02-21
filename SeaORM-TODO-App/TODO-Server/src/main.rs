mod fruits_table;
mod insert_values;
mod server;
mod suppliers_table;
mod todos_table;

pub use fruits_table::prelude::*;
pub use insert_values::*;
pub use server::*;
pub use suppliers_table::prelude::*;
pub use todos_table::prelude::*;

use async_std::sync::Arc;
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
    let db = Arc::new(db);

    insert_fruits(&db).await?;
    insert_suppliers(&db).await?;

    start_server(db).await?;

    Ok(())
}
