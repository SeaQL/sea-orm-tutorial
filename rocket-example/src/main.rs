mod entities;
mod migrator;
mod setup;

use entities::{prelude::*, *};
use migrator::Migrator;
use rocket::{
    fs::{relative, FileServer},
    *,
};
use rocket_dyn_templates::Template;
use sea_orm::*;
use sea_orm_migration::MigratorTrait;
use serde_json::json;
use setup::set_up_db;

#[get("/")]
fn index() -> Template {
    Template::render("index", json!({}))
}

#[get("/bakeries")]
async fn bakeries(db: &State<DatabaseConnection>) -> Result<Template, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let bakeries = Bakery::find()
        .all(db)
        .await
        .map_err(Into::into)?
        .into_iter()
        .map(|b| json!({ "name": b.name, "id": b.id }))
        .collect::<Vec<_>>();

    Ok(Template::render(
        "bakeries",
        json!({ "bakeries": bakeries, "num_bakeries": bakeries.len() }),
    ))
}

#[get("/bakeries/<id>")]
async fn bakery_by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Template, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let bakery = Bakery::find_by_id(id).one(db).await.map_err(Into::into)?;

    Ok(if let Some(bakery) = bakery {
        Template::render(
            "bakery",
            json!({ "id": bakery.id, "name": bakery.name, "profit_margin": bakery.profit_margin }),
        )
    } else {
        return Err(format!("No bakery with id {id} is found.").into());
    })
}

#[get("/new")]
fn new() -> Template {
    Template::render("new", json!({}))
}

// Use `GET` to support query parameters to simplify things
#[get("/bakeries?<name>&<profit_margin>")]
async fn new_bakery(
    db: &State<DatabaseConnection>,
    name: &str,
    profit_margin: Option<f64>,
) -> Result<Template, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let profit_margin = profit_margin.unwrap_or_default();

    let new_bakery = bakery::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        profit_margin: ActiveValue::Set(profit_margin),
        ..Default::default()
    };

    Bakery::insert(new_bakery)
        .exec(db)
        .await
        .map_err(Into::into)?;

    Ok(Template::render(
        "success",
        json!({ "name": name, "profit_margin": profit_margin}),
    ))
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

    rocket::build()
        .manage(db)
        .mount("/", FileServer::from(relative!("/static")))
        .mount(
            "/",
            routes![index, bakeries, bakery_by_id, new, new_bakery, reset],
        )
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        json! ({
            "uri": req.uri()
        }),
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
