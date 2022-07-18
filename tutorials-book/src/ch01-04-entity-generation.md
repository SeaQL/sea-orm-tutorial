# Generate Entity from Database

Now that we have a database with a defined schema, we can generate the entities with `sea-orm-cli`.

`sea-orm-cli` is able to discover the schema given the database URL and generated the appropriate entity files.

```sh
# In case you have not installed `sea-orm-cli`
$ cargo install sea-orm-cli
```

```sh
# Generate entity files of database `bakeries_db` to `src/entities`
$ sea-orm-cli generate entity \
    -u mysql://root:password@localhost:3306/bakeries_db \
    -o src/entities
```

The generated entity files:

```
bakery-backend
├── Cargo.toml
├── migration
│   └── ...
└── src
    ├── entities
    │   ├── baker.rs
    │   ├── bakery.rs
    │   ├── mod.rs
    │   └── prelude.rs
    └── main.rs
```

Put the focus on `baker.rs` and `bakery.rs`, they are the entities representing the tables `Baker` and `Bakery`, respectively.
