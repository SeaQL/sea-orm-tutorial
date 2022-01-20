# SeaORM Concepts

#### Schema - A database with tables

In SeaORM, a database with a collection of tables is called a `Schema`. Sea-ORM derives the database name from the provided database URL and the `sea_orm::Database::connect(&url)` method as shown below:

```rust
// The MySQL database URL for the database `fruit_markets`
let database_url = "mysql://sea:sea@localhost/time_tests";

// The PostgreSQL database URL for the database `fruit_markets`
let database_url = "postgres://sea:sea@localhost/time_tests";

// The SQLite database URL for the database `fruit_markets`
let database_url = "sqlite://sea:sea@localhost/time_tests";

// The connection to the provided database selected by the `database_url` variable
 let dbconn = sea_orm::Database::connect(database_url).await.unwrap();
```

#### Entity - a table

An `Entity` in SeaORM represents a database table  which helps you perform `CRUD` (Create, Read, Update, and Delete) operations on relevant tables. An entity implements the `EntityTrait` which provides an API for you to inspect its [Column](https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure#column), [PrimaryKey](https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure#primary-key) and a [Relation](https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure#relation) . Each column in a table is referred to as `attribute` . See [https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/#column](https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/#column) for more information of the structure of an `Entity`.

For more details on the concepts of SeaORM, visit [https://www.sea-ql.org/SeaORM/docs/introduction/sea-orm](https://www.sea-ql.org/SeaORM/docs/introduction/sea-orm)
