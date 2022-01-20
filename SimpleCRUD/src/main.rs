mod fruits_table;
use fruits_table::prelude::Fruits;

// Import the needed modules for table creation
use sea_orm::{ConnectionTrait, Database, Schema};
// Handle errors using the `https://crates.io/crates/anyhow` crate
use anyhow::Result;

// Convert this main function into async function
#[async_std::main]
async fn main() -> Result<()> {
    let env_database_url = include_str!("../.env").trim();
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    let database_url = split_url[1];

    let db = Database::connect(database_url).await.unwrap();

    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let create_table_op = db
        .execute(builder.build(&schema.create_table_from_entity(Fruits)))
        .await;
    println!(
        "`CREATE TABLE fruits` {:?}",
        match create_table_op {
            Ok(_) => "Operation Successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error {:?}", e),
        }
    );
    Ok(())
}
