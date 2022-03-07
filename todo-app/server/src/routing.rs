use crate::{Fruits, Todos, TodosActiveModel, TodosColumn, DATABASE_CONNECTION};
use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Store {
    username: String,
    todo_list: String,
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
