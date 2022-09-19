# Query with GraphQL

To support queries, we extend the `QueryRoot` struct:

## Basic Queries

```rust, no_run
// src/schema.rs

- use async_graphql::Object;
+ use async_graphql::{Context, Object};
+ use sea_orm::*;

+ use crate::entities::{prelude::*, *};

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    ...

    // For finding all bakeries
+   async fn bakeries(&self, ctx: &Context<'_>) -> Result<Vec<bakery::Model>, DbErr> {
+       let db = ctx.data::<DatabaseConnection>().unwrap();
+       Bakery::find().all(db).await
+   }

    // For finding one bakery by id
+   async fn bakery(&self, ctx: &Context<'_>, id: i32) -> Result<Option<bakery::Model>, DbErr> {
+       let db = ctx.data::<DatabaseConnection>().unwrap();
+
+       Bakery::find_by_id(id).one(db).await
+   }
}
```

Example queries:

```
GraphQL Request:
{
  bakeries {
    name
  }
}

Response:
{
  "data": {
    "bakeries": [
      {
        "name": "ABC Bakery"
      },
      {
        "name": "La Boulangerie"
      },
      {
        "name": "Sad Bakery"
      }
    ]
  }
}
```

```
GraphQL Request:
{
  bakery(id: 1) {
    name
  }
}

Response:
{
  "data": {
    "bakery": {
      "name": "ABC Bakery"
    }
  }
}
```

_If `name` is replaced by other fields of `bakery::Model`, the requests will automatically be supported. This is because `bakery::Model` derives from `async_graphql::SimpleObject` in the previous section._

## Relational Query

One of the most appealing features of GraphQL is its convenient support for relational queries.

Recall that a Bakery may hire many chefs. We can give `bakery::Model` ComplexObject support to allow for this relational query.

```rust, no_run
// src/entities/bakery.rs

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
+ #[graphql(complex, name = "Bakery")]
#[sea_orm(table_name = "bakery")]
pub struct Model {

...
```

```rust, no_run
// src/schema.rs

- use async_graphql::{Context, Object};
+ use async_graphql::{ComplexObject, Context, Object};

...

+ #[ComplexObject]
+ impl bakery::Model {
+   async fn chefs(&self, ctx: &Context<'_>) -> Result<Vec<chef::Model>, DbErr> {
+       let db = ctx.data::<DatabaseConnection>().unwrap();
+
+       self.find_related(Chef).all(db).await
+   }
+ }
```

Example query:

```
GraphQL Request:
{
  bakery(id: 1) {
    name,
    chefs {
      name
    }
  }
}

Response:
{
  "data": {
    "bakery": {
      "name": "ABC Bakery",
      "chefs": [
        {
          "name": "Sanford"
        },
        {
          "name": "Billy"
        }
      ]
    }
  }
}
```
