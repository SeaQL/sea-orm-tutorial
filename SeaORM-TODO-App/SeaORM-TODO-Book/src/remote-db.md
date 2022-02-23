# Remote Database Operations

To persist data remotely in the PostgreSQL database, modify the `src/db_ops.rs` file and add thew following code

`File: src/db_ops.rs`

```rust,no_run,noplayground
// -- code snippet --
use crate::{
    synching_to_server, Command, MemDB, MyTodos, MyTodosActiveModel, MyTodosModel, TodoList,
};

pub async fn get_fruits() -> anyhow::Result<Vec<String>> {
    // Get the fruits first
    let get_fruits = Command::ListFruits;
    let serialized_command = bincode::serialize(&get_fruits)?;
    let mut fruits_list: Vec<String>;

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(&serialized_command).await?;

    let mut fruits_buf = vec![0u8; 4096];
    loop {
        let n = stream.read(&mut fruits_buf).await?;
        let rx: Vec<_> = bincode::deserialize(&fruits_buf).unwrap();

        fruits_list = rx;

        if n != 0 {
            break;
        }
    }

    Ok(fruits_list)
}

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

pub async fn get(db: &DatabaseConnection) -> Result<Vec<MyTodosModel>, sea_orm::DbErr> {
    MyTodos::find().all(db).await
}

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

pub async fn update_remote_storage(memdb: &MemDB, username: &str) -> anyhow::Result<()> {
    let mut temp_list = TodoList::default();
    memdb.lock().await.values().for_each(|todo| {
        if todo.status == 0 {
            temp_list.queued.push(todo.to_owned());
        } else {
            temp_list.completed.push(todo.to_owned());
        }
    });

    synching_to_server();

    // Update a todo_list
    let update_todo = Command::UpdateTodoList {
        username: username.to_owned(),
        todo_list: serde_json::to_string(&temp_list)?,
    };
    let serialized_command = bincode::serialize(&update_todo)?;

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(&serialized_command).await?;

    let mut buffer = vec![0u8; 4096];
    stream.read(&mut buffer).await?;

    bincode::deserialize::<String>(&buffer)?;

    Ok(())
}

pub async fn get_user_remote_storage(username: &str) -> anyhow::Result<Option<String>> {
    let get_user = Command::Get(username.to_owned());
    let serialized_command = bincode::serialize(&get_user)?;

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(&serialized_command).await?;

    let mut buffer = vec![0u8; 4096];
    stream.read(&mut buffer).await?;

    Ok(bincode::deserialize::<Option<String>>(&buffer)?)
}

pub async fn create_new_user(username: &str) -> anyhow::Result<String> {
    let create_user = Command::CreateUser(username.to_owned());
    let serialized_command = bincode::serialize(&create_user)?;

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(&serialized_command).await?;

    let mut buffer = vec![0u8; 4096];
    stream.read(&mut buffer).await?;

    Ok(bincode::deserialize::<String>(&buffer)?)
}

```

`get_fruits()` queries the list of fruits from the remote database.

`store()` will persist the contents of the in-memory database to local SQLite cache.

`load_sqlite_cache()` queries the local database a list of TODOs. This is useful when the client starts, since it fetches the cached TODOs and loads them into the in-memory database `MemDB`.

`edit()` persists the edits to the TODOs to the SQLite cache.

`done()` persists the state of the in-memory database with the `completed` TODOs in the SQLite cache.

`undo()` persists the state of the in-memory database with the `queued` TODOs in the SQLite cache reflecting the TODOs which have been moved from the `completed` Vector to the `queued` Vector.

`update_remote_storage()` updates the remote PostgreSQL database with the new changes in the TODO list.

`get_user_remote_storage()` checks if the username provided is in the remote PostgreSQL database.

`create_new_user()` creates a new user in the remote PostgreSQL database with the given `username`.

Up next is reading from the terminal and performing database operations bases on the command.
