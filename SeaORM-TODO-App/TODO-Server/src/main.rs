use sea_orm::{
    sea_query::{Alias, ColumnDef, Table},
    ConnectionTrait, Database, DbBackend,
};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let db_postgres = DbBackend::Postgres;

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

    // Read the database environment from the `.env` file
    let env_database_url = include_str!("../../postgresql_config.env").trim();
    // Split the env url
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    // Get item with the format `database_backend://username:password@localhost/database`
    let database_url = split_url[1];

    let db = Database::connect(database_url).await?;

    let create_table_op = db.execute(db_postgres.build(&fruits_table)).await;
    println!(
        "`CREATE TABLE fruits` {:?}",
        match create_table_op {
            Ok(_) => "Operation Successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error {:?}", e),
        }
    );

    Ok(())
}
