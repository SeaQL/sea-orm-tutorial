use crate::{Fruits, FruitsActiveModel, Suppliers, SuppliersActiveModel};
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

// Insert suppliers in the `suppliers` table
pub async fn insert_fruits(db: &DatabaseConnection) -> anyhow::Result<()> {
    // Get current system time
    let now = Utc::now();

    // Convert system time to `NaiveDateTime` since SeaORM `DateTime` expects this;
    let naive_system_time = now.naive_utc();

    let apple = FruitsActiveModel {
        fruit_name: Set("Apple".to_owned()),
        date_time: Set(naive_system_time),
        ..Default::default()
    };

    let orange = FruitsActiveModel {
        fruit_name: Set("Orange".to_owned()),
        date_time: Set(naive_system_time),
        ..Default::default()
    };

    let mango = FruitsActiveModel {
        fruit_name: Set("Mango".to_owned()),
        date_time: Set(naive_system_time),
        ..Default::default()
    };

    let pineapple = FruitsActiveModel {
        fruit_name: Set("Pineapple".to_owned()),
        date_time: Set(naive_system_time),
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

    // Convert system time to `NaiveDateTime` since SeaORM `DateTime` expects this;
    let naive_system_time = now.naive_utc();

    let john_doe = SuppliersActiveModel {
        suppliers_name: Set("John Doe".to_owned()),
        fruit_id: Set(1),
        date_time: Set(naive_system_time),
        ..Default::default()
    };

    let jane_doe = SuppliersActiveModel {
        suppliers_name: Set("Jane Doe".to_owned()),
        fruit_id: Set(2),
        date_time: Set(naive_system_time),
        ..Default::default()
    };

    let doe_junior = SuppliersActiveModel {
        suppliers_name: Set("Doe Junior".to_owned()),
        fruit_id: Set(3),
        date_time: Set(naive_system_time),
        ..Default::default()
    };

    let doe_senior = SuppliersActiveModel {
        suppliers_name: Set("Doe Senior".to_owned()),
        fruit_id: Set(4),
        date_time: Set(naive_system_time),
        ..Default::default()
    };

    let suppliers_insert_operation =
        Suppliers::insert_many(vec![john_doe, jane_doe, doe_senior, doe_junior])
            .exec(db)
            .await;

    println!("INSERTED SUPPLIERS: {:?}", suppliers_insert_operation?);

    Ok(())
}
