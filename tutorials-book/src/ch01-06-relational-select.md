# Relational Select

In the previous section, we explored how to [perform select on a single entity](ch01-05-basic-crud-operations.md#find-single-entity).

However, relational databases are known for connecting entities with relations, such that we can perform queries **across different entities**.

For example, given a bakery, we can find all the chefs working there.

Suppose the following code were run before, inserting the bakery and the chefs it employed into the database.

```rust, no_run
let la_boulangerie = bakery::ActiveModel {
    name: ActiveValue::Set("La Boulangerie".to_owned()),
    profit_margin: ActiveValue::Set(0.0),
    ..Default::default()
};
let bakery_res = Bakery::insert(la_boulangerie).exec(&db).await?;

for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
    let chef = chef::ActiveModel {
        name: ActiveValue::Set(chef_name.to_owned()),
        bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
        ..Default::default()
    };
    Chef::insert(chef).exec(&db).await?;
}
```

There are 4 chefs working at the bakery _La Boulangerie_, and we can find them later on as follows:

```rust, no_run
// First find *La Boulangerie* as a Model
let la_boulangerie: bakery::Model = Bakery::find_by_id(bakery_res.last_insert_id)
    .one(&db)
    .await?
    .unwrap();

let chefs: Vec<chef::Model> = la_boulangerie.find_related(Chef).all(&db).await?;
let mut chef_names: Vec<String> = chefs.into_iter().map(|b| b.name).collect();
chef_names.sort_unstable();

assert_eq!(chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
```

As new bakeries open in the town, it would be inefficient to do `find_related` on each of the bakeries. 
```rust, no_run
    // Inserting two bakeries and their chefs
    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::Set("La Boulangerie".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;
    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }
    let la_id = bakery_res.last_insert_id;

    let arte_by_padaria = bakery::ActiveModel {
        name: ActiveValue::Set("Arte by Padaria".to_owned()),
        profit_margin: ActiveValue::Set(0.2),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(arte_by_padaria).exec(db).await?;
    for chef_name in ["Brian", "Charles", "Kate", "Samantha"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }
    let arte_id = bakery_res.last_insert_id;

    // would then need two sets of find_related to find 
```

We can utilize a loader to do the heavy lifting for us.

```rust, no_run
    // First find bakeries as Models
    let bakeries: Vec<bakery::Model> = Bakery::find()
        .filter(
            Condition::any()
                .add(bakery::Column::Id.eq(la_id))
                .add(bakery::Column::Id.eq(arte_id))
        )
        .all(db)
        .await?;

    // Then use loader to load the chefs in one query.
    let chefs: Vec<Vec<chef::Model>> = bakeries.load_many(Chef, db).await?;
    let mut la_chef_names: Vec<String> = chefs[0].to_owned().into_iter().map(|b| b.name).collect();
    la_chef_names.sort_unstable();
    let mut arte_chef_names: Vec<String> = chefs[1].to_owned().into_iter().map(|b| b.name).collect();
    arte_chef_names.sort_unstable();

    assert_eq!(la_chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
    assert_eq!(arte_chef_names, ["Brian", "Charles", "Kate", "Samantha"]);
```

Using a loader can greatly reduce the traffic of the database.

For more advanced usage, visit the [documentation](https://www.sea-ql.org/SeaORM/docs/basic-crud/select/#find-related-models).
