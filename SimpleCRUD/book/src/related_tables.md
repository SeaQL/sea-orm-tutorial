# SELECTing related tables

SeaORM makes it easy to fetch a table and it's related table referenced by its primary key using the `Entity::find().find_with_related(Other_Entity).all(DatabaseConnection)` chain of methods.

```rust
// --- Code Snippet ---
#[async_std::main]
async fn main() -> Result<()> {
    let env_database_url = include_str!("../.env").trim();
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    let database_url = split_url[1];

    let db = Database::connect(database_url).await?;
    
    // -- Code snippet --
    

    let supplier_insert_operation =
        Suppliers::insert_many(vec![supplier_01, supplier_02, supplier_03])
            .exec(&db)
            .await;

    println!("INSERTED MANY: {:?}", supplier_insert_operation?);
    
+   let who_supplies = Suppliers::find().find_with_related(Fruits).all(&db).await?;
+   dbg!(&who_supplies);
    
    Ok(())
    
}
```

The operation returns a `Vec` which contains a tuple `(Model, Vec<Model>) ` which is `Vec<(Model, Vec<Model>)>`.  This means that the first `Model` , `tuple.0` is the `Model` that has relationships with the other `Model`s in the `tuple.1` index which is `Vec<Model>` .

Running the program, prints:

```sh
$
[
    (
        Model {
            supplier_id: 1,
            supplier_name: "John Doe",
            fruit_id: 1,
        },
        [
            Model {
                fruit_id: 1,
                name: "Apple",
                datetime_utc: 2022-01-26T09:16:43,
                unit_price: 2,
                sku: "FM2022AKB40",
            },
        ],
    ),
    (
        Model {
            supplier_id: 2,
            supplier_name: "Jane Doe",
            fruit_id: 2,
        },
        [
            Model {
                fruit_id: 2,
                name: "Banana",
                datetime_utc: 2022-01-26T09:16:43,
                unit_price: 2,
                sku: "FM2022AKB41",
            },
        ],
    ),
    (
        Model {
            supplier_id: 3,
            supplier_name: "Junior Doe",
            fruit_id: 3,
        },
        [
            Model {
                fruit_id: 3,
                name: "Pineapple",
                datetime_utc: 2022-01-26T09:16:43,
                unit_price: 10,
                sku: "FM2022AKB42",
            },
        ],
    ),
]
```

---

Thats SeaORM in action. A beginner friendly ORM, one codebase for MySQL, SQLite, MariaDB and PostgreSQL. What else could you ask for :)
