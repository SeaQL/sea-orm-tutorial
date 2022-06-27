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
