# Remote Database Operations

### Data Persistence to the local database

First, import necessary dependencies

`File: src/db_ops.rs`

```rust,no_run,noplayground
+ use serde::{Serialize, Deserialize};
+ use crate::{synching_to_server, MemDB, MyTodos, MyTodosActiveModel, MyTodosModel};

```

#### Fetching the fruits

The `get_fruits()` function fetches the list of fruits from the remote PostgreSQL database using HTTP at the `/fruits` route. The response `response.as_str()?` is serialized using `serde_json` to reate the `fruits` list.

`File: src/db_ops.rs`

```rust,no_run,noplayground
pub async fn get_fruits() -> anyhow::Result<Vec<String>> {
    let response = minreq::get("http://127.0.0.1:8080/fruits").send()?;

    let fruits_list: Vec<String> = serde_json::from_str(&response.as_str()?)?;

    Ok(fruits_list)
}
```

#### Storing the fruits in local cache

The `store()` function takes a `DatabaseConnection` , `quantity` and `todo_name` as arguments and creates an `ActiveModel` as defined by `MyTodosActiveModel` which is then inserted into the SQLite cache using`MyTodos::insert()`.

`File: src/db_ops.rs`

```rust,no_run,noplayground
pub async fn store(db: &DatabaseConnection, quantity: &str, todo_name: &str) -> anyhow::Result<()> {
    let my_todo = MyTodosActiveModel {
        todo_name: Set(todo_name.to_owned()),
        quantity: Set(quantity.to_owned()),
        status: Set(0),
        ..Default::default()
    };

    MyTodos::insert(my_todo).exec(db).await?;

    Ok(())
}
```

#### Fetching the TODO Models from the local SQLite cache

`get()` function fetches all the `TODO` models using `MyTodos::find().all()` returning all the fetched Models as `Vec<Model>`

`File: src/db_ops.rs`

```rust,no_run,noplayground
pub async fn get(db: &DatabaseConnection) -> Result<Vec<MyTodosModel>, sea_orm::DbErr> {
    MyTodos::find().all(db).await
}
```

#### Performing modifications on the local SQLite cache

The `edit()` , `done()` and `undo()` functions perform modifications to the SQLite data. The `edit()` function modifies a TODO in the queue by changing it's `quantity`. The `done()` function moves an incomplete todo from the `queued` field of the `TodoList` struct into the `completed` field of the `TodoList` struct while the `undo()` function does the opposite, moving a TODO from the `completed` field to the `queued` field of the `TodoList` struct.

`File: src/db_ops.rs`

```rust,no_run,noplayground
pub async fn edit(
    db: &DatabaseConnection,
    todo_model: &MyTodosModel,
    quantity: String,
) -> Result<MyTodosModel, sea_orm::DbErr> {
    let mut todos_active_model: MyTodosActiveModel = todo_model.to_owned().into();
    todos_active_model.quantity = Set(quantity);

    Ok(todos_active_model.update(db).await?)
}

pub async fn done(
    db: &DatabaseConnection,
    todo_model: &MyTodosModel,
) -> Result<MyTodosModel, sea_orm::DbErr> {
    let mut todos_active_model: MyTodosActiveModel = todo_model.to_owned().into();
    todos_active_model.status = Set(1);

    Ok(todos_active_model.update(db).await?)
}

pub async fn undo(
    db: &DatabaseConnection,
    todo_model: &MyTodosModel,
) -> Result<MyTodosModel, sea_orm::DbErr> {
    let mut todos_active_model: MyTodosActiveModel = todo_model.to_owned().into();
    todos_active_model.status = Set(0);

    Ok(todos_active_model.update(db).await?)
}
```

#### Initializing the In-memory database with the SQLite cache

Sometimes the `client` might not exit gracefully using the `EXIT` command, this prevents the `client` from syncing the cache with the remote database. The `load_sqlite_cache()` function loads the SQLite cache into in-memory database `MemDB`. It iterates the result of the `get()` function and uses the `todo_name` field of the `MyTodosModel` as the `key` of the `MemDB`.

```rust,no_run,noplayground
pub(crate) async fn load_sqlite_cache(
    db: &DatabaseConnection,
    memdb: &mut MemDB,
) -> Result<(), sea_orm::DbErr> {
    let sqlite_cache = get(&db).await?;
    memdb.lock().await.clear();
    for mytodo_model in sqlite_cache {
        memdb
            .lock()
            .await
            .insert(mytodo_model.todo_name.clone(), mytodo_model);
    }

    Ok(())
}
```

#### Updating the remote database on graceful exit.

The `update_remote_storage()` uses the `username` and contents of the `MemDB` to update the remote database over HTTP protocol. The `TodoList` struct is first initialized and the contents of the `MemDB` are sorted into `queued` and `completed` TODOs and then converted into JSON string. The `username` and this JSON string are also converted into JSON using the `json` crate and then sent to the remote server using `minreq` at the `/update_todo` route. If the HTTP response status code is   `500` and the matching body data is `ome("MODEL_NOT_FOUND")`, then another request is made to the `/store` route where a new `username` is created and the `todo_list` added under that username.

```rust,no_run,noplayground
pub async fn update_remote_storage(memdb: &MemDB, username: &str) -> anyhow::Result<()> {
    let mut temp_list = TodoList::default();
    memdb.lock().await.values().for_each(|todo| {
        if todo.status == 0 {
            temp_list.queued.push(todo.to_owned());
        } else {
            temp_list.completed.push(todo.to_owned());
        }
    });

    let todo_list = serde_json::to_string(&temp_list)?;

    synching_to_server();

    let response = minreq::post("http://127.0.0.1:8080/update_todo")
        .with_header("Content-Type", "application/json")
        .with_body(
            json::object! {
                username: username,
                todo_list: todo_list.clone(),
            }
            .dump(),
        )
        .send()?;

    if response.status_code == 500 {
        let body = serde_json::from_str::<Option<String>>(&response.as_str()?)?;
        if body == Some("MODEL_NOT_FOUND".to_owned()) {
            minreq::post("http://127.0.0.1:8080/store")
                .with_header("Content-Type", "application/json")
                .with_body(
                    json::object! {
                        username: username,
                        todo_list: todo_list,
                    }
                    .dump(),
                )
                .send()?;
        }
    }

    Ok(())
}

```



Up next is reading from the terminal and performing database operations bases on the command.
