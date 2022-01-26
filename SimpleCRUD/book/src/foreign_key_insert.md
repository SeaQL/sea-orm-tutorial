# Inserting Values into a Table with a Foreign Key

Insert many suppliers in the `supplies` table

```rust
// -- code snippet --

// Convert this main function into async function
#[async_std::main]
async fn main() -> Result<()> {
    // -- code snippet --
    
+   let supplier_01 = SuppliersActiveModel {
+       supplier_name: Set("John Doe".to_owned()),
+       fruit_id: Set(1_i32),
+       ..Default::default()
+   };

+   let supplier_02 = SuppliersActiveModel {
+       supplier_name: Set("Jane Doe".to_owned()),
+       fruit_id: Set(2_i32),
+       ..Default::default()
+   };

+   let supplier_03 = SuppliersActiveModel {
+       supplier_name: Set("Junior Doe".to_owned()),
+       fruit_id: Set(3_i32),
+       ..Default::default()
+   };

+   let supplier_insert_operation =
+       Suppliers::insert_many(vec![supplier_01, supplier_02, supplier_03])
+           .exec(&db)
+           .await;

+   println!("INSERTED MANY: {:?}", supplier_insert_operation?);

    
 	Ok(())   
}
```



Executing the program returns

```sh
$ INSERTED MANY: InsertResult { last_insert_id: 1 }
```

