# Delete Operation


To perform a delete operation in SeaORM, first, fetch the row to perform the operation using `Model` convert it into an `ActiveModel` by calling the `into()` methof on the `Model` , perform the operation on the field on the `ActiveModel` and then and then call the `.delete()` method on the `ActiveModel` or use `Fruit::delete()`. The executed result returns the  `Model` that was updated if successful.

```rust
//-- snippet --

+ use sea_orm::sea_query::{Expr, Value}; // Types necessary to perform updates and conversions between types

#[async_std::main]
async fn main() -> Result<()> {
    let env_database_url = include_str!("../.env").trim();
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    let database_url = split_url[1];

    let db = Database::connect(database_url).await.unwrap();
    
    ...

   // Delete the `mango` row

+  let find_mango = Fruits::find()
+       .filter(FruitsColumn::Name.contains("mango"))
+       .one(&db)
+       .await;
    
+   if let Some(mango_model) = find_mango.unwrap() {
+       println!(
+           "DELETED MANGO: {:?}",
+           mango_model.delete(&db).await.unwrap()
+       );
+   } else {
+       println!("`Mango` row not found");
+   }

    Ok(())
}
```

Running the program returns 

```sh
$ DELETED MANGO: DeleteResult { rows_affected: 1 }
```

