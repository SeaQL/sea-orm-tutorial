use crate::{Fruits, Todos, TodosActiveModel, TodosColumn, TodosModel};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ServerErrors {
    InvalidCommand,
    ModelNotFound,
}

impl Error for ServerErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ServerErrors::InvalidCommand => Some(&crate::ServerErrors::InvalidCommand),
            ServerErrors::ModelNotFound => Some(&crate::ServerErrors::ModelNotFound),
        }
    }
}

impl fmt::Display for ServerErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                ServerErrors::InvalidCommand => "Invalid command provided",
                ServerErrors::ModelNotFound => "The result of the query is `None`",
            }
        )
    }
}

// The commands to use to perform CRUD operations on PostgreSQL
#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Store { username: String, todo_list: String },
    UpdateTodoList { username: String, todo_list: String },
    Get(String),
    CreateUser(String),
    ListFruits,
}

impl Command {
    pub async fn get_fruits(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        let fruit_models = Fruits::find().all(db).await?;
        let fruits = fruit_models
            .iter()
            .map(|fruit_model| fruit_model.fruit_name.clone())
            .collect::<Vec<String>>();

        Ok(bincode::serialize(&fruits)?)
    }

    pub async fn store(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::Store {
                username,
                todo_list,
            } => {
                let todo_user = TodosActiveModel {
                    username: Set(username.to_owned()),
                    todo_list: Set(Some(todo_list.to_owned())),
                    ..Default::default()
                };
                Todos::insert(todo_user).exec(db).await?;

                Ok(bincode::serialize("INSERTED")?)
            }
            _ => Err(anyhow::Error::new(ServerErrors::InvalidCommand)),
        }
    }

    pub async fn create_new_user(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::CreateUser(username) => {
                let todo_user = TodosActiveModel {
                    username: Set(username.to_owned()),
                    ..Default::default()
                };
                Todos::insert(todo_user).exec(db).await?;

                Ok(bincode::serialize(&format!("CREATED_USER `{}`", username))?)
            }
            _ => Err(anyhow::Error::new(ServerErrors::InvalidCommand)),
        }
    }

    pub async fn get_user_todo(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::Get(user) => {
                let get_todo = Todos::find()
                    .filter(TodosColumn::Username.contains(user))
                    .one(db)
                    .await?;

                if let Some(found_todo) = get_todo {
                    Ok(bincode::serialize(&found_todo.todo_list)?)
                } else {
                    Ok(bincode::serialize(&Some("USER_NOT_FOUND"))?)
                }
            }
            _ => Err(anyhow::Error::new(ServerErrors::InvalidCommand)),
        }
    }

    pub async fn update_todo_list(&self, db: &DatabaseConnection) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::UpdateTodoList {
                username,
                todo_list,
            } => {
                let found_todo: Option<TodosModel> = Todos::find()
                    .filter(TodosColumn::Username.contains(username))
                    .one(db)
                    .await?;

                match found_todo {
                    Some(todo_model) => {
                        let mut todo_model: TodosActiveModel = todo_model.into();
                        todo_model.todo_list = Set(Some(todo_list.to_owned()));
                        todo_model.update(db).await?;
                    }
                    None => return Err(anyhow::Error::new(ServerErrors::ModelNotFound)),
                };

                Ok(bincode::serialize("UPDATED_TODO")?)
            }
            _ => Err(anyhow::Error::new(ServerErrors::InvalidCommand)),
        }
    }
}
