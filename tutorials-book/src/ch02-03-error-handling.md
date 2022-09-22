# Error Handling

First, define a [custom responder](https://rocket.rs/v0.5-rc/guide/responses/#custom-responders):

```rust, no_run
// src/main.rs

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder {
    message: String,
}

// The following impl's are for easy conversion of error types.

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

```

To catch and handle the errors:

```rust, no_run
// src/main.rs

#[get("/bakeries")]
async fn bakeries(db: &State<DatabaseConnection>)
- -> Json<Vec<String>>
+ -> Result<Json<Vec<String>>, ErrorResponder>
{
    let db = db as &DatabaseConnection;

    let bakery_names = Bakery::find()
        .all(db)
        .await
-       .unwrap()
+       .map_err(Into::into)?
        .into_iter()
        .map(|b| b.name)
        .collect::<Vec<String>>();

-   Json(bakery_names)
+   Ok(Json(bakery_names))
}
```
