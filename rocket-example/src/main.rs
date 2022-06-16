mod entities;
mod migrator;
mod setup;

use entities::{prelude::*, *};
use migrator::Migrator;
use rocket::{serde::json::Json, *};
use sea_orm::*;
use sea_orm_migration::MigratorTrait;
use setup::set_up_db;

#[get("/")]
async fn index() -> &'static str {
    "Hello, bakeries!"
}

#[get("/bakeries")]
async fn bakeries(db: &State<DatabaseConnection>) -> Result<Json<Vec<String>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let bakery_names = Bakery::find()
        .all(db)
        .await
        .map_err(Into::into)?
        .into_iter()
        .map(|b| b.name)
        .collect::<Vec<String>>();

    Ok(Json(bakery_names))
}

#[get("/bakeries/<id>")]
async fn bakery_by_id(db: &State<DatabaseConnection>, id: i32) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let bakery = Bakery::find_by_id(id).one(db).await.map_err(Into::into)?;

    Ok(if let Some(bakery) = bakery {
        bakery.name
    } else {
        return Err(format!("No bakery with id {id} is found.").into());
    })
}

#[post("/bakeries/<name>?<profit_margin>")]
async fn new_bakery(
    db: &State<DatabaseConnection>,
    name: &str,
    profit_margin: Option<f64>,
) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_bakery = bakery::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        profit_margin: ActiveValue::Set(profit_margin.unwrap_or_default()),
        ..Default::default()
    };

    Bakery::insert(new_bakery)
        .exec(db)
        .await
        .map_err(Into::into)?;

    Ok(())
}

#[post("/reset")]
async fn reset(db: &State<DatabaseConnection>) -> Result<(), ErrorResponder> {
    Migrator::refresh(db).await.map_err(Into::into)?;

    Ok(())
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build().manage(db).mount(
        "/",
        routes![index, bakeries, bakery_by_id, new_bakery, reset],
    )
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder {
    message: String,
}

#[allow(clippy::from_over_into)]
impl Into<ErrorResponder> for DbErr {
    fn into(self) -> ErrorResponder {
        ErrorResponder {
            message: self.to_string(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ErrorResponder> for String {
    fn into(self) -> ErrorResponder {
        ErrorResponder { message: self }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ErrorResponder> for &str {
    fn into(self) -> ErrorResponder {
        self.to_owned().into()
    }
}
