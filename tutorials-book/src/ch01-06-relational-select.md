# Relational Select

In the previous section, we explored how to [perform select on a single entity](ch01-05-basic-crud-operations.md#find-single-entity).

However, relational databases are known for connecting entities with relations, such that we can perform queries **across different entities**.

For example, given a bakery, we can find all the bakers working there.

Suppose the following code were run before, inserting the bakery and the bakers it employed into the database.

```rust, no_run
let la_boulangerie = bakery::ActiveModel {
    name: ActiveValue::Set("La Boulangerie".to_owned()),
    profit_margin: ActiveValue::Set(0.0),
    ..Default::default()
};
let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;

for baker_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
    let baker = baker::ActiveModel {
        name: ActiveValue::Set(baker_name.to_owned()),
        bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
        ..Default::default()
    };
    Baker::insert(baker).exec(db).await?;
}
```

There are 4 bakers working at the bakery *La Boulangerie*, and we can find them later on as follows:

```rust, no_run
// First find *La Boulangerie* as a Model
let la_boulangerie: bakery::Model = Bakery::find_by_id(bakery_res.last_insert_id)
    .one(db)
    .await?
    .unwrap();

let bakers: Vec<baker::Model> = la_boulangerie.find_related(Baker).all(db).await?;
let mut baker_names: Vec<String> = bakers.into_iter().map(|b| b.name).collect();
baker_names.sort_unstable();

assert_eq!(baker_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
```

For more advanced usage, visit the [documentation](https://www.sea-ql.org/SeaORM/docs/basic-crud/select/#find-related-models).
