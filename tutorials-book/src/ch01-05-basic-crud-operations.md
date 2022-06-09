# Basic CRUD Operations

In this section, we showcase how to perform basic operations with the schema we've defined.

## `use` the Entities

The entities are the Rust representation of the tables in the database. SeaORM enables us to make use of those entities to perform operations on the database programmatically.

```rust, no_run
// src/main.rs

+ mod entities;

...

+ use entities::*;

+ use entities::{prelude::*, *};

...
```

## Insert and Update

Insert and update operations can be performed using `ActiveModel` of the entities.

Let's insert a new bakery called *Happy Bakery* into our `Bakery` table.

```rust, no_run
// src/main.rs

...

use sea_orm::*;

...

async fn run() -> Result<(), DbErr> {
    
    ...

    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(db).await?;
}
```

Suppose, later on, the owner of *Happy Bakery* adopts a brand new perspective of life, and renames it to *Sad Bakery*.

We can perform the update as follows:

```rust, no_run
let sad_bakery = bakery::ActiveModel {
    id: ActiveValue::Set(res.last_insert_id),
    name: ActiveValue::Set("Sad Bakery".to_owned()),
    profit_margin: ActiveValue::NotSet,
};
sad_bakery.update(db).await?;
```

Let's welcome John, the first employee of *Sad Bakery*!

```rust, no_run
let john = baker::ActiveModel {
    name: ActiveValue::Set("John".to_owned()),
    bakery_id: ActiveValue::Set(res.last_insert_id), // a foreign key
    ..Default::default()
};
Baker::insert(john).exec(db).await?;
```

## Find (single entity)

We can find all or some of the bakeries in the database as follows:

```rust, no_run
// Finding all is built-in
let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
assert_eq!(bakeries.len(), 1);

// Finding by id is built-in
let sad_bakery: Option<bakery::Model> = Bakery::find_by_id(1).one(db).await?;
assert_eq!(sad_bakery.unwrap().name, "Sad Bakery");

// Finding by arbitrary column with `filter()`
let sad_bakery: Option<bakery::Model> = Bakery::find()
    .filter(bakery::Column::Name.eq("Sad Bakery"))
    .one(db)
    .await?;
assert_eq!(sad_bakery.unwrap().id, 1);
```

For relational select on multiple entities, visit the next [section](ch01-06-relational-select.md).

## Delete

Sadly, *Sad Bakery* is unable to survive in the rapidly changing economy; it has been forced to liquidate!

We have no choice but to remove its entry in our database:

```rust, no_run
let john = baker::ActiveModel {
    id: ActiveValue::Set(1), // The primary must be set
    ..Default::default()
};
john.delete(db).await?;

let sad_bakery = bakery::ActiveModel {
    id: ActiveValue::Set(1), // The primary must be set
    ..Default::default()
};
sad_bakery.delete(db).await?;

let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
assert!(bakeries.is_empty());
```
