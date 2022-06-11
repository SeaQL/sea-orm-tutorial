# Update Operation



To perform an update in SeaORM, first, fetch the row to perform the operation using `Model` convert it into an `ActiveModel` by calling the `into()` methof on the `Model` , perform the operation on the field on the `ActiveModel` and then call the `.update()` method on the `ActiveModel`. The executed result returns the  `Model` that was updated if successful.

```rust,no_run
//-- snippet --

+ use sea_orm::sea_query::{Expr, Value}; // Types necessary to perform updates and conversions between types

#[async_std::main]
async fn main() -> Result<()> {
    let env_database_url = include_str!("../.env").trim();
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    let database_url = split_url[1];

    let db = Database::connect(database_url).await?;
    
    ...

    let find_pineapple = Fruits::find()
    	.filter(FruitsColumn::Name.contains("pineapple"))
    	.one(&db)
    	.await?;
-   println!("{:?}", find_pineapple?);
+   println!("{:?}", find_pineapple.as_ref()); // Reference the `Model` instead of owning it
    
    // Update the `pineapple` column with a new unit price
+   if let Some(pineapple_model) = find_pineapple {
+       let mut pineapple_active_model: FruitsActiveModel = pineapple_model.into();
+       pineapple_active_model.unit_price = Set(10);

+       let updated_pineapple_model: FruitsModel =
+          pineapple_active_model.update(&db).await?;

+       println!("UPDATED PRICE: {:?}", updated_pineapple_model);
+   } else {
+       println!("`Pineapple` fruit not found");
+   }

    Ok(())
}
```

Running the program returns 

```sh
$ UPDATED PRICE: Model { fruit_id: 3, name: "Pineapple", datetime_utc: 2022-01-22T13:35:27, unit_price: 10, sku: "FM2022AKB42" }
```

