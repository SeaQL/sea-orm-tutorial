use crate::{synching_to_server, MemDB, MyTodos, MyTodosActiveModel, MyTodosModel, TodoList};
use sea_orm::{
    sea_query::{Alias, ColumnDef, Table},
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Set,
};

pub async fn create_todo_table(db: &DatabaseConnection) -> anyhow::Result<()> {
    let database_backend = db.get_database_backend();
    // Create the `todos` table
    let todos_table = Table::create()
        .table(Alias::new("todo_list"))
        .if_not_exists()
        .col(
            ColumnDef::new(Alias::new("todo_id"))
                .integer()
                .primary_key()
                .not_null()
                .auto_increment(),
        )
        .col(
            ColumnDef::new(Alias::new("todo_name"))
                .string()
                .unique_key()
                .not_null(),
        )
        .col(ColumnDef::new(Alias::new("quantity")).string().not_null())
        .col(ColumnDef::new(Alias::new("status")).boolean().not_null())
        .to_owned();

    // Executing the SQL query to create the `todos` table in SQLite
    let create_table_op = db.execute(database_backend.build(&todos_table)).await;
    // Print the result in a user friendly way
    println!(
        "`CREATE TABLE todo_list` {:?}",
        match create_table_op {
            Ok(_) => "Operation Successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error {:?}", e),
        }
    );

    Ok(())
}

pub async fn get_fruits() -> anyhow::Result<Vec<String>> {
    let response = minreq::get("http://127.0.0.1:8080/fruits").send()?;

    let fruits_list: Vec<String> = serde_json::from_str(&response.as_str()?)?;

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
