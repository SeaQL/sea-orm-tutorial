use crate::{MemDB, MyTodos, MyTodosActiveModel, MyTodosModel};
use sea_orm::{
    sea_query::{Alias, ColumnDef, ForeignKey, ForeignKeyAction, Table},
    ActiveModelTrait, ConnectionTrait, Database, DatabaseConnection, EntityTrait, Set,
};

pub async fn database_config() -> Result<DatabaseConnection, sea_orm::DbErr> {
    // Read the database environment from the `.env` file
    let env_database_url = include_str!("../.env").trim();
    // Split the env url
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    // Get item with the format `database_backend://username:password@localhost/database`
    let database_url = split_url[1];

    Database::connect(database_url).await
}

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
    let create_table_op = db.execute(database_backend.build(&todos_table)).await;

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
