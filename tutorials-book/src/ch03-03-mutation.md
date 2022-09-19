# Mutation

## Preparation

To support mutations with GraphQL, we need to create a struct to serve as the root, as for queries.

```rust, no_run
// src/schema.rs

...

pub(crate) struct QueryRoot;
+ pub(crate) struct MutationRoot;

...
```

```rust, no_run
// src/main.rs

...

- use async_graphql::{EmptyMutation, EmptySubscription, Schema};
+ use async_graphql::{EmptySubscription, Schema};

...

- type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
+ type SchemaType = Schema<QueryRoot, MutationRoot, EmptySubscription>;

...

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

-   let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
+   let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
       .data(db) // Add the database connection to the GraphQL global context
       .finish();

...
```

## Define resolvers

Define the mutation resolvers just like the ones for queries:

```rust, no_run
// src/schema.rs

...

#[Object]
impl MutationRoot {
    // For inserting a bakery
    async fn add_bakery(&self, ctx: &Context<'_>, name: String) -> Result<bakery::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Bakery::insert(bakery::ActiveModel {
            name: ActiveValue::Set(name),
            profit_margin: ActiveValue::Set(0.0),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Bakery::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|b| b.unwrap())
    }

    // For inserting a chef
    async fn add_chef(
        &self,
        ctx: &Context<'_>,
        name: String,
        bakery_id: i32,
    ) -> Result<chef::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Chef::insert(chef::ActiveModel {
            name: ActiveValue::Set(name),
            bakery_id: ActiveValue::Set(bakery_id),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Chef::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|b| b.unwrap())
    }
}
```

Examples:

```
GraphQL Request:
mutation {
  addChefy(name: "Excellent Bakery") {
    id,
    name,
    profitMargin
  }
}

Response:
{
  "data": {
    "addChefy": {
      "id": 4,
      "name": "Excellent Bakery",
      "profitMargin": 0
    }
  }
}
```

```
GraphQL Request:
mutation {
  addChef(name: "Chris", bakeryId: 1) {
    id,
    name,
    bakery {
      chefs {
        name
      }
    }
  }
}

Response:
{
  "data": {
    "addChef": {
      "id": 3,
      "name": "Chris",
      "bakery": {
        "chefs": [
          {
            "name": "Sanford"
          },
          {
            "name": "Billy"
          },
          {
            "name": "Chris"
          }
        ]
      }
    }
  }
}
```
