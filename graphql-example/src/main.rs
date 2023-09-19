mod entities;
mod migrator;
mod schema;
mod setup;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_rocket::*;
use rocket::{response::content, *};
use schema::*;
use sea_orm::DbErr;
use setup::set_up_db;

type SchemaType = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[get("/")]
fn index() -> String {
    "Hello, bakeries!".to_owned()
}

#[rocket::get("/graphql")]
fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<SchemaType>, request: GraphQLRequest) -> GraphQLResponse {
    let schema = schema as &SchemaType;

    request.execute(schema).await
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    rocket::build()
        .manage(schema)
        .mount("/", routes![index, graphql_playground, graphql_request])
        .register("/", catchers![not_found])
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> String {
    format!("{} not found.", req.uri())
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder {
    message: String,
}

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}
