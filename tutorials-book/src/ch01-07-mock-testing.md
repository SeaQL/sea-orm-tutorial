# Testing with Mock Interface

In some cases, we want to verify the application logic without using a real database. As such, SeaORM provides a `MockDatabase` interface to be used in development.

For example, we don't want to set up and use a real database for unit testing because the database layer should be independent of the application logic layer. Using a mock interface provides stable and correct behavior in the database layer, hence any errors that emerge can only be due to bugs in the application logic layer.

Also, a real database may not be preferred when we want to maximize the portability of the development environment. Using a mock interface effective takes away the need for setting up and maintaining a real database, therefore application logic developers can do their work virtually anywhere.

## Add the `mock` Cargo feature

```diff
// Cargo.toml

...

- sea-orm = { version = "0.8.0", features = [ ... ] } 
+ sea-orm = { version = "0.8.0", features = [ ... , "mock" ] }

...
```

## Define the expected query results

First, we define what we want our mock database to return.

Note that the function `append_query_results()` takes a vector of vectors, where each vector nested inside represent the result of a single query.

```rust, no_run
let db: &DatabaseConnection = &MockDatabase::new(DatabaseBackend::MySql)
    .append_query_results(vec![
        // First query result
        vec![bakery::Model {
            id: 1,
            name: "Happy Bakery".to_owned(),
            profit_margin: 0.0,
        }],
        // Second query result
        vec![
            bakery::Model {
                id: 1,
                name: "Happy Bakery".to_owned(),
                profit_margin: 0.0,
            },
            bakery::Model {
                id: 2,
                name: "Sad Bakery".to_owned(),
                profit_margin: 100.0,
            },
            bakery::Model {
                id: 3,
                name: "La Boulangerie".to_owned(),
                profit_margin: 17.89,
            },
        ],
    ])
    .append_query_results(vec![
        // Third query result
        vec![
            baker::Model {
                id: 1,
                name: "Jolie".to_owned(),
                contact_details: None,
                bakery_id: 3,
            },
            baker::Model {
                id: 2,
                name: "Charles".to_owned(),
                contact_details: None,
                bakery_id: 3,
            },
            baker::Model {
                id: 3,
                name: "Madeleine".to_owned(),
                contact_details: None,
                bakery_id: 3,
            },
            baker::Model {
                id: 4,
                name: "Frederic".to_owned(),
                contact_details: None,
                bakery_id: 3,
            },
        ]
    ])
    .into_connection();
```

*Note: if a query result contains multiple models (like the second and third ones above) and `Entity::find().one(db)` is called, only the first one will be returned. The rest of the models in the query will be discarded.*

## Use the returned query results

Then the query results can be mocked and passed to other parts of the application logic.

```rust, no_run
let happy_bakery: Option<bakery::Model> = Bakery::find().one(db).await?;
assert_eq!(
    happy_bakery.unwrap(),
    bakery::Model {
        id: 1,
        name: "Happy Bakery".to_owned(),
        profit_margin: 0.0,
    }
);

let all_bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
assert_eq!(
    all_bakeries,
    vec![
        bakery::Model {
            id: 1,
            name: "Happy Bakery".to_owned(),
            profit_margin: 0.0,
        },
        bakery::Model {
            id: 2,
            name: "Sad Bakery".to_owned(),
            profit_margin: 100.0,
        },
        bakery::Model {
            id: 3,
            name: "La Boulangerie".to_owned(),
            profit_margin: 17.89,
        },
    ]
);

let la_boulangerie_bakers: Vec<baker::Model> = Baker::find().all(db).await?;
assert_eq!(
    la_boulangerie_bakers,
    vec![
        baker::Model {
            id: 1,
            name: "Jolie".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
        baker::Model {
            id: 2,
            name: "Charles".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
        baker::Model {
            id: 3,
            name: "Madeleine".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
        baker::Model {
            id: 4,
            name: "Frederic".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
    ]
);
```

## Mock execution results

To mock the results of CRUD operations, we can use `append_exec_results()`.

As it is highly similar to the above, it won't be covered in detail in this tutorial. For more information, refer to the [documentation](https://www.sea-ql.org/SeaORM/docs/write-test/mock/#mocking-execution-result).
