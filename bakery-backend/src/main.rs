mod entities;
mod migrator;

use entities::{prelude::*, *};
use futures::executor::block_on;
use migrator::Migrator;
use sea_orm::*;
use sea_orm_migration::prelude::*;

const DATABASE_URL: &str = "mysql://root:root@localhost:3306";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db_name = "bakeries_db";
    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };
    let schema_manager = SchemaManager::new(db); // To investigate the schema

    Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("bakery").await?);
    assert!(schema_manager.has_table("baker").await?);

    // Insert and Update
    {
        let happy_bakery = bakery::ActiveModel {
            name: ActiveValue::Set("Happy Bakery".to_owned()),
            profit_margin: ActiveValue::Set(0.0),
            ..Default::default()
        };
        let res = Bakery::insert(happy_bakery).exec(db).await?;

        let sad_bakery = bakery::ActiveModel {
            id: ActiveValue::Set(res.last_insert_id),
            name: ActiveValue::Set("Sad Bakery".to_owned()),
            profit_margin: ActiveValue::NotSet,
        };
        sad_bakery.update(db).await?;

        let john = baker::ActiveModel {
            name: ActiveValue::Set("John".to_owned()),
            bakery_id: ActiveValue::Set(res.last_insert_id),
            ..Default::default()
        };
        Baker::insert(john).exec(db).await?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
