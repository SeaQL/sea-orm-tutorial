# Connect to Database

First, we define a function to help us create a database and/or connect to it.

It is basically the same as in [Section 1.1](ch01-01-project-setup.html#creating-a-database).

```rust, no_run
// src/setup.rs

use sea_orm::*;

// Replace with your database URL
const DATABASE_URL: &str = "mysql://root:root@localhost:3306";

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    // Replace with your desired database name
    let db_name = "bakeries_db";
    let db = match db.get_database_backend() {
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

    Ok(db)
}
```

We instruct Rocket to manage the database connection as a [state](https://rocket.rs/v0.5-rc/guide/state/#state).

```rust, no_run
// src/main.rs

#[launch]
fn rocket() -> _ {
+   let db = match set_up_db().await {
+       Ok(db) => db,
+       Err(err) => panic!("{}", err),
+   };

    rocket::build()
+       .manage(db)
        .mount("/", routes![index, bakeries])
}
```

The database connection can then be accessed and used as in [previous sections](ch01-05-basic-crud-operations.md).

```rust, no_run
// src/main.rs

use rocket::serde::json::Json;

...

#[get("/bakeries")]
async fn bakeries(db: &State<DatabaseConnection>) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;

    let bakery_names = Bakery::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| b.name)
        .collect::<Vec<String>>();

    Json(bakery_names)
}

...

#[launch]
fn rocket() -> _ {
   rocket::build()
    .mount(
        "/",
        // Don't forget to mount the new endpoint handlers
        routes![
            index,
+           bakeries
        ]
    )
}
```

To verify it works:

```sh
$ cargo run
```

```
GET localhost:8000/bakeries

["Bakery Names", "In The", "Database", "If Any"]
```
