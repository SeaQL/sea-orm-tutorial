# Project Setup

## Create a Rocket application

The initial setup of this chapter is vastly similar to that of the previous chapter.

Refer to [Section 2.1](ch02-01-project-setup.md) and [Section 2.2](ch02-02-connect-to-database.md) to create a Rocket application and configure the database connection.

## Set up `async_graphql` support

Add the crates as dependencies:

```diff
// Cargo.toml

...

[dependencies]
+ async-graphql = "4.0.4"
+ async-graphql-rocket = "4.0.4"

...
```

Make sure the entities are generated ([Section 1.4](ch01-04-entity-generation.md)), and extend them to support basic GraphQL queries by attributes:

```rust, no_run
// src/entities/chef.rs

+ use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

- #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
+ #[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "chef")]
pub struct Model {

...
```

```rust, no_run
// src/entities/bakery.rs

+ use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

- #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
+ #[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "bakery")]
pub struct Model {

...
```

Create a struct to serve as the root of queries. The root level query requests will be defined here:

```rust, no_run
// src/schema.rs

use async_graphql::Object;

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello GraphQL".to_owned()
    }
}
```

Build the `Schema` and attach it to Rocket as a state, and create an endpoint to serve GraphQL requests:

```rust, no_run
// src/main.rs

mod entities;
mod migrator;
+ mod schema;
mod setup;

+ use async_graphql::{EmptyMutation, EmptySubscription, Schema};
+ use async_graphql_rocket::*;
use rocket::*;
+ use schema::*;
use sea_orm::*;
use setup::set_up_db;

+ type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

...

+ #[rocket::post("/graphql", data = "<request>", format = "application/json")]
+ async fn graphql_request(schema: &State<SchemaType>, request: GraphQLRequest) -> GraphQLResponse {
+    request.execute(schema).await
+ }

...

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    // Build the Schema
+   let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
+       .data(db) // Add the database connection to the GraphQL global context
+       .finish();

    rocket::build()
-       .manage(db) // db is now managed by schema
+       .manage(schema) // schema is managed by rocket
-       .mount("/", routes![index])
+       .mount("/", routes![index, graphql_request])
        .register("/", catchers![not_found])
}
...
```

To verify it works:

```sh
$ cargo run
```

For debugging, GraphQL requests can be sent via the [GraphQL Playground](ch03-04-graphql-playground.md).

```
GraphQL Request:
{
  hello
}

Response:
{
  "data": {
    "hello": "Hello GraphQL"
  }
}
```
