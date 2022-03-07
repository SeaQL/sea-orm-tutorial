use crate::{Fruits, FruitsActiveModel};
use sea_orm::{DatabaseConnection, EntityTrait, Set};

// Insert suppliers in the `suppliers` table
pub async fn insert_fruits(db: &DatabaseConnection) -> anyhow::Result<()> {
    let apple = FruitsActiveModel {
        fruit_name: Set("Apple".to_owned()),
        ..Default::default()
    };

    let orange = FruitsActiveModel {
        fruit_name: Set("Orange".to_owned()),
        ..Default::default()
    };

    let mango = FruitsActiveModel {
        fruit_name: Set("Mango".to_owned()),
        ..Default::default()
    };

    let pineapple = FruitsActiveModel {
        fruit_name: Set("Pineapple".to_owned()),
        ..Default::default()
    };

    let fruit_insert_operation = Fruits::insert_many(vec![apple, orange, mango, pineapple])
        .exec(db)
        .await;

    println!("INSERTED FRUITS: {:?}", fruit_insert_operation?);

    Ok(())
}
