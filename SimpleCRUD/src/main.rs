mod fruits_table;
use fruits_table::prelude::*;

// Import the needed modules for table creation
use sea_orm::{entity::Set, prelude::*, ConnectionTrait, Database, Schema};
// Handle errors using the `https://crates.io/crates/anyhow` crate
use anyhow::Result;
use chrono::Utc;

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

    // Get current system time
    let now = chrono::offset::Utc::now();

    // Convert system time to `NaiveDateTime` since SeaORM `DateTime` expects this;
    let naive_system_time = now.naive_utc();

    let fruit_01 = FruitsActiveModel {
        name: Set("Apple".to_owned()),
        datetime_utc: Set(naive_system_time),
        unit_price: Set(2),
        sku: Set("FM2022AKB40".to_owned()),
        ..Default::default()
    };
    let fruit_insert_operation = Fruits::insert(fruit_01).exec(&db).await;

    println!("INSERTED ONE: {:?}", fruit_insert_operation.unwrap());

    let fruit_02 = FruitsActiveModel {
        name: Set("Banana".to_owned()),
        datetime_utc: Set(Utc::now().naive_utc()),
        unit_price: Set(2),
        sku: Set("FM2022AKB41".to_owned()),
        ..Default::default()
    };

    let fruit_03 = FruitsActiveModel {
        name: Set("Pineapple".to_owned()),
        datetime_utc: Set(Utc::now().naive_utc()),
        unit_price: Set(8),
        sku: Set("FM2022AKB42".to_owned()),
        ..Default::default()
    };

    let fruit_04 = FruitsActiveModel {
        name: Set("Mango".to_owned()),
        datetime_utc: Set(Utc::now().naive_utc()),
        unit_price: Set(6),
        sku: Set("FM2022AKB43".to_owned()),
        ..Default::default()
    };
    let fruit_insert_operation = Fruits::insert_many(vec![fruit_02, fruit_03, fruit_04])
        .exec(&db)
        .await;

    println!("INSERTED ONE: {:?}", fruit_insert_operation.unwrap());

    let fruits_table_rows = Fruits::find().all(&db).await;
    println!("{:?}", fruits_table_rows.unwrap());

    let fruits_by_id = Fruits::find_by_id(2).one(&db).await;
    println!("{:?}", fruits_by_id.unwrap());

    let find_pineapple = Fruits::find()
        .filter(FruitsColumn::Name.contains("pineapple"))
        .one(&db)
        .await;
    println!("{:?}", find_pineapple.as_ref().unwrap());

    // Update the `pineapple` column with a new unit price
    if let Some(pineapple_model) = find_pineapple.unwrap() {
        let mut pineapple_active_model: FruitsActiveModel = pineapple_model.into();
        pineapple_active_model.unit_price = Set(10);

        let updated_pineapple_model: FruitsModel =
            pineapple_active_model.update(&db).await.unwrap();

        println!("UPDATED PRICE: {:?}", updated_pineapple_model.clone());
    } else {
        println!("`Pineapple` column not found");
    }

    // Delete the `mango` column

    let find_mango = Fruits::find()
        .filter(FruitsColumn::Name.contains("mango"))
        .one(&db)
        .await;

    if let Some(mango_model) = find_mango.unwrap() {
        println!(
            "DELETED MANGO: {:?}",
            mango_model.delete(&db).await.unwrap()
        );
    } else {
        println!("`Mango` column not found");
    }

    Ok(())
}
