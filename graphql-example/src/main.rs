mod entities;
mod migrator;
mod schema;
mod setup;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_rocket::*;
use rocket::*;
use schema::*;
use sea_orm::DbErr;
use setup::set_up_db;

type SchemaType = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[get("/")]
fn index() -> String {
    "Hello, bakeries!".to_owned()
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<SchemaType>, request: GraphQLRequest) -> GraphQLResponse {
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
        .mount("/", routes![index, graphql_request])
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
