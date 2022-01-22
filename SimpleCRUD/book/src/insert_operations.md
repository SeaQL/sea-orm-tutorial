# Insert Operation

SeaORM insert and read operations are done using the `Entity` derived from the `Model` struct using the `EntityTrait`. 

Let's insert a fruit `Apple` with a unit price per Kg of $2 and an SKU of `FM2022AKB40`.

Add `chrono` crate to get the current time from  the system time

```toml
[package]
name = "simple-crud"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0.52"
async-std = { version = "1.10.0", features = ["attributes"] }
sea-orm = { version = "0.5.0", features = [
    "runtime-async-std-rustls",
    "sqlx-mysql",
    "macros",
], default-features = false }
+ chrono = "0.4.19" # Add chrono here
```

Modify the current `sea-orm` features to add the feature `with-chrono`. This activates Date and Time features.

```TOML
[package]
name = "simple-crud"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0.52"
async-std = { version = "1.10.0", features = ["attributes"] }
chrono = "0.4.19"
sea-orm = { version = "0.5.0", features = [
    "runtime-async-std-rustls",
    "sqlx-mysql",
    "macros",
+   "with-chrono", # New feature
], default-features = false }
chrono = "0.4.19" # Add chrono here
```



Next, call `Utc::now()` chrono method to get the system time and then import     sea_orm::entity::`Set`  to perform convertions of the Rust data types into SQL ready data type `ActiveValue`

```rust
// -- code snippet --
+ use sea_orm::entity::Set;

#[async_std::main]
async fn main() -> Result<()>{
	...

    // Get current system time
+   let now = chrono::offset::Utc::now();

    // Convert system time to `NaiveDateTime` since SeaORM `DateTime` expects this;
+   let naive_system_time = now.naive_utc();
    
+   let fruit_01 = FruitsActiveModel {
+       name: Set("Apple".to_owned()),
+       datetime_utc: Set(naive_system_time),
+       unit_price: Set(2),
+       sku: Set("FM2022AKB40".to_owned()),
+       ..Default::default()
+   };
+   let fruit_insert_operation = Fruits::insert(fruit_01).exec(&db).await;
    
+   println!("INSERTED ONE: {:?}", fruit_insert_operation.unwrap());
    
    Ok(())
}
```

Since an `Entity` implements `EntityTrait`, the insert method is availabe. executing `Fruits::insert(fruit_01)`  will perform the operation on the database using `exec(&db).await`. Here, the `insert` operation inserts only one row into the specified database;

Running the program using `cargo run` should print

```sh
$ INSERTED ONE: InsertResult { last_insert_id: 1 }
```

Let's insert more than one row at a time using the  `Fruits::insert_many()` method.

```rust
// -- code snippet --
+ use chrono::offset::Utc;
#[async_std::main]
async fn main() -> Result<()>{
	...
    
+   let fruit_02 = FruitsActiveModel {
+       name: Set("Banana".to_owned()),
+       datetime_utc: Set(Utc::now().naive_utc()),
+       unit_price: Set(2),
+       sku: Set("FM2022AKB41".to_owned()),
+       ..Default::default()
+   };
    
+   let fruit_03 = FruitsActiveModel {
+       name: Set("Pineapple".to_owned()),
+       datetime_utc: Set(Utc::now().naive_utc()),
+       unit_price: Set(8),
+       sku: Set("FM2022AKB42".to_owned()),
+       ..Default::default()
+   };
    
+   let fruit_04 = FruitsActiveModel {
+       name: Set("Mango".to_owned()),
+       datetime_utc: Set(Utc::now().naive_utc()),
+       unit_price: Set(6),
+       sku: Set("FM2022AKB43".to_owned()),
+       ..Default::default()
+   };
+   let fruit_insert_operation = Fruits::insert_many(vec![fruit_02, fruit_03, fruit_04]).exec(&db).await;
    
+   println!("INSERTED MANY: {:?}", fruit_insert_operation.unwrap());
    
    Ok(())
}
```

Running the program with `cargo run` prints

```sh
$ INSERTED MANY: InsertResult { last_insert_id: 3 }
```



Next up is reading one value or many values from a table.
