# Web API Integration

We can wrap more of the backend's functionalities into our Rocket application.

Check [Rocket's official documentation](https://rocket.rs/v0.5-rc/guide/) for how to use their interfaces.

Below are some examples: *(Don't forget to mount all new handlers in `rocket()`!)*

## Fetch one Bakery by id

```rust, no_run
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
```

## Add a new Bakery

Query parameters are used for input here for simplicity. Alternatively, use [Body Data](https://rocket.rs/v0.5-rc/guide/requests/#body-data).

```rust, no_run
use entities::*;

#[post("/bakeries?<name>&<profit_margin>")]
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
```
