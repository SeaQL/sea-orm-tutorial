# Read Operation

SeaORM can perform read operations through the `Entity::find()` method.

#### Find all rows using in a table

The `.all()` method in `Entity` is used to fetch all rows in a table.

```rust
//-- snippet --

#[async_std::main]
async fn main() -> Result<()> {
    let env_database_url = include_str!("../.env").trim();
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    let database_url = split_url[1];

    let db = Database::connect(database_url).await.unwrap();
    
    ...

+   let fruits_table_rows = Fruits::find().all(&db).await;
+   println!("{:?}", fruits_table_rows.unwrap());

    Ok(())
}
```

To fetch all the rows inside a table, in this case `Fruits`, call the `.all()` method on `Fruits::find()`

This should print all the rows in the table `fruits` to the console as an array of `Model`s.

```sh
$ [Model { fruit_id: 1, name: "Apple", datetime_utc: 2022-01-22T10:36:39, unit_price: 2, sku: "FM2022AKB40" }, Model { fruit_id: 2, name: "Banana", datetime_utc: 2022-01-22T10:36:39, unit_price: 2, sku: "FM2022AKB41" }, Model { fruit_id: 3, name: "Pineapple", datetime_utc: 2022-01-22T10:36:39, unit_price: 8, sku: "FM2022AKB42" }, Model { fruit_id: 4, name: "Mango", datetime_utc: 2022-01-22T10:36:39, unit_price: 6, sku: "FM2022AKB43" }]
```



#### Find one row by the primary key

Call the `.find_by_id(primary_key)` on `Fruits` entity (table).

```rust
//-- snippet --

#[async_std::main]
async fn main() -> Result<()> {
    let env_database_url = include_str!("../.env").trim();
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    let database_url = split_url[1];

    let db = Database::connect(database_url).await.unwrap();
    
    ...

+   let fruits_by_id = Fruits::find_by_id(2).one(&db).await;
+   println!("{:?}", fruits_by_id.unwrap());

    Ok(())
}
```

The `.one()` method is used to retrieve one `Model` that matches the query instead of a `Vec<Model>` like the `.all()` method. `.one()` method returns an `Option<Model>` where `Some(Model)` is returned if the `Model` exists or a `None` is returned if a `Model` doesn't exist.

Running the program prints

```sh
$ Some(Model { fruit_id: 2, name: "Banana", datetime_utc: 2022-01-22T10:36:39, unit_price: 2, sku: "FM2022AKB41" })
```



#### Find and Filter a Row by Column Name

Calling `filter()` method on `Entity::find()` returns a `Model` containing the matching row.

```rust
//-- snippet --

#[async_std::main]
async fn main() -> Result<()> {
    let env_database_url = include_str!("../.env").trim();
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    let database_url = split_url[1];

    let db = Database::connect(database_url).await.unwrap();
    
    ...

+   let find_pineapple = Fruits::find()
+    	.filter(FruitsColumn::Name.contains("pineapple"))
+    	.one(&db)
+    	.await;
+   println!("{:?}", find_pineapple.unwrap());

    Ok(())
}
```
The `FruitsColumn::Name` is a `Column` that was autoderived by SeaORM from the `Model` struct fields, which we imported and renamed using `use super::fruits::Column as FruitsColumn` in the previous section. `.contains()` method on `FruitsColumn` allows filtering of the `Model`with `Pineapple` as it's name. Note that this is case insensitive so even calling `.contains(piNeApPle)` will yield the same results.

Running the program prints:

```sh
$ Some(Model { fruit_id: 3, name: "Pineapple", datetime_utc: 2022-01-22T10:36:39, unit_price: 8, sku: "FM2022AKB42" })
```

