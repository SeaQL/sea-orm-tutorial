# Building Server Connections and Responses

Create a new file in the `src` folder called `routing.rs`.

Then register the module to the `src.main.rs` file

```rust,no_run,noplayground
// -- code snippet --

mod routing;
pub use routing::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
	// -- code snippet --
	
	Ok(())
}
```



#### The HTTP API

The HTTP API will take the following routes :

`/` route to indicate that the server is online.

`/fruits` to fetch a list of all the fruits in the database.

`/store` to insert a  `username` and `todo_list` in the database.

`/update_todo` to perform an update to the `todo_list`.

Create these routes in the `connection.rs` file.

`File: todo-app/src/connection.rs`

```rust,no_run,noplayground
use crate::{Fruits, Todos, TodosActiveModel, TodosColumn, DATABASE_CONNECTION};
use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Store {
    username: String,
    todo_list: String,
}

```

Here, the `Store` struct is used to handle all incoming JSON data in a `POST` request. It deserializes the `username` and `todo_list` from a JSON string.

#### Responsders

The `/` route will be handled by the function `root()`

```rust,no_run.noplayground
pub async fn root() -> &'static str {
    "Remote PostgreSQL Server Online!"
}
```

The `/fruits` route will be handled by the function `get_fruits()`.

```rust,no_run,noplayground
pub async fn get_fruits() -> impl IntoResponse {
    let db = DATABASE_CONNECTION.get().unwrap();

    match Fruits::find().all(db).await {
        Ok(fruit_models) => {
            let fruits = fruit_models
                .iter()
                .map(|fruit_model| fruit_model.fruit_name.clone())
                .collect::<Vec<String>>();

            (StatusCode::ACCEPTED, Json(fruits))
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(vec![error.to_string()]),
        ),
    }
}
```

This function finds all the `fruit model`s in the `fruits` table, iterates over those models if any is found and takes the `fruit_name`s field of the model collecting them into a `Vec<String>` , parses them into a JSON string and returns it as the  result together with status code `201` from the `StatusCode::ACCEPTED` enum from the `http` crate re-exported by `axum`.  In case of an error from the database, the error is converted into a JSON string using `Json(vec![error.to_string()]),` and returned together with a status code of `StatusCode::INTERNAL_SERVER_ERROR` of the `http` crate. This is a HTTP error `500`.

Next, is the `insert` and `update` operations handled by the `store_todo()` and `update_todo()` functions respectively.

```rust,no_run,noplayground

pub async fn store_todo(Json(payload): Json<Store>) -> impl IntoResponse {
    let db = DATABASE_CONNECTION.get().unwrap();

    let todo_user = TodosActiveModel {
        username: Set(payload.username.to_owned()),
        todo_list: Set(Some(payload.todo_list.to_owned())),
        ..Default::default()
    };

    match Todos::insert(todo_user).exec(db).await {
        Ok(_) => (StatusCode::ACCEPTED, Json("INSERTED".to_owned())),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())),
    }
}

pub async fn update_todo(Json(payload): Json<Store>) -> impl IntoResponse {
    let db = DATABASE_CONNECTION.get().unwrap();

    match Todos::find()
        .filter(TodosColumn::Username.contains(&payload.username))
        .one(db)
        .await
    {
        Ok(found_model) => {
            if let Some(model) = found_model {
                let mut todo_model: TodosActiveModel = model.into();
                todo_model.todo_list = Set(Some(payload.todo_list.to_owned()));
                match todo_model.update(db).await {
                    Ok(_) => (StatusCode::NO_CONTENT, Json("UPDATED_TODO".to_owned())),
                    Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())),
                }
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("MODEL_NOT_FOUND".to_owned()),
                )
            }
        }

        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())),
    }
}
```

This functions are similar and have the same structure as the `get_fruits()` function in terms of their generated results and the error handling. However, the `store_todo()` function responds with the JSON string `INSERTED` while the `update_todo()` function responds with `UPDATED_TODO` JSON string in case of a successful update or a `MODEL_NOT_FOUND` JSON string in case the `username` that is being updated does not exist.

Now that we have established how the API will be access and perform database operations, add the code to the source file.

`File: todo-app/src/routing.rs`

```rust,no_run,noplayground
use crate::{Fruits, Todos, TodosActiveModel, TodosColumn, DATABASE_CONNECTION};
use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Store {
    username: String,
    todo_list: String,
}

#[derive(Deserialize, Debug)]
pub struct GetUser {
    username: String,
}

pub async fn root() -> &'static str {
    "Remote PostgreSQL Server Online!"
}

pub async fn get_fruits() -> impl IntoResponse {
    let db = DATABASE_CONNECTION.get().unwrap();

    match Fruits::find().all(db).await {
        Ok(fruit_models) => {
            let fruits = fruit_models
                .iter()
                .map(|fruit_model| fruit_model.fruit_name.clone())
                .collect::<Vec<String>>();

            (StatusCode::ACCEPTED, Json(fruits))
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(vec![error.to_string()]),
        ),
    }
}

pub async fn store_todo(Json(payload): Json<Store>) -> impl IntoResponse {
    let db = DATABASE_CONNECTION.get().unwrap();

    let todo_user = TodosActiveModel {
        username: Set(payload.username.to_owned()),
        todo_list: Set(Some(payload.todo_list.to_owned())),
        ..Default::default()
    };

    match Todos::insert(todo_user).exec(db).await {
        Ok(_) => (StatusCode::ACCEPTED, Json("INSERTED".to_owned())),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())),
    }
}

pub async fn update_todo(Json(payload): Json<Store>) -> impl IntoResponse {
    let db = DATABASE_CONNECTION.get().unwrap();

    match Todos::find()
        .filter(TodosColumn::Username.contains(&payload.username))
        .one(db)
        .await
    {
        Ok(found_model) => {
            if let Some(model) = found_model {
                let mut todo_model: TodosActiveModel = model.into();
                todo_model.todo_list = Set(Some(payload.todo_list.to_owned()));
                match todo_model.update(db).await {
                    Ok(_) => (StatusCode::NO_CONTENT, Json("UPDATED_TODO".to_owned())),
                    Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())),
                }
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("MODEL_NOT_FOUND".to_owned()),
                )
            }
        }

        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())),
    }
}

```

Lastly, add the routes to the `main.rs` file.

`File: todo-app/src/main.rs`

```rust,no_run,noplayground
  use dotenv::dotenv;
  use once_cell::sync::OnceCell;
  use sea_orm::{
      sea_query::{Alias, ColumnDef, Table},
      ConnectionTrait, Database, DatabaseConnection, DbBackend,
  };
+ use std::net::SocketAddr;
+ use axum::{
+     routing::{get, post},
+     Router,
+ };

#[tokio::main]
async fn main() {
+    let app = Router::new()
+        .route("/", get(root))
+        .route("/fruits", get(get_fruits))
+        .route("/get_user", post(get_user))
+        .route("/store", post(store_todo))
+        .route("/update_todo", post(update_todo));

+    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
+    println!("listening on http://{}", addr);
+    axum::Server::bind(&addr)
+        .serve(app.into_make_service())
+        .await?;
    
	Ok(())
}
        
```



Run the program using `cargo run`. It print the following to the terminal

```sh
$ Running `/media/su43/IGIED-01/Rust-Projects/SeaQL/todo-app/target/debug/server`
`CREATE TABLE fruits` "Operation Successful"
`CREATE TABLE todos` "Operation Successful"
INSERTED FRUITS: InsertResult { last_insert_id: 1 }
```

The server is now listening on `127.0.0.1:8080` for incoming `HTTP requests`.

Next, we build the client.
