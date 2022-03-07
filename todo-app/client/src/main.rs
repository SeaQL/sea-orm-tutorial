mod db_ops;
mod handler;
mod todo_list_table;
mod user_input;
mod utils;

pub use db_ops::*;
pub use handler::*;
pub use todo_list_table::prelude::*;
pub use user_input::*;
pub use utils::*;

use dotenv::dotenv;
use sea_orm::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Read the database environment from the `.env` file
    let database_url = dotenv::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;

    create_todo_table(&db).await?;

    input_handler(&db).await?;

    Ok(())
}
