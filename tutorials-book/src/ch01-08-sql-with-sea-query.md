# Optional: Building SQL Queries with SeaQuery

If you prefer the flexibility of SQL, you can use [SeaQuery](https://crates.io/crates/sea-query) to build SQL-like statements for any queries or operations.

SeaQuery is built-in for SeaORM, so no extra setup is required.

## Insert statements

Raw SQL:

```sql
INSERT INTO `bakery` (`name`, `profit_margin`) VALUES ('SQL Bakery', -100)
```

SeaQuery:

```rust, no_run
let columns: Vec<Alias> = ["name", "profit_margin"]
    .into_iter()
    .map(Alias::new)
    .collect();

let mut stmt = Query::insert();
stmt.into_table(bakery::Entity).columns(columns);

// Invoke `values_panic()` for each row
stmt.values_panic(["SQL Bakery".into(), (-100.0).into()]);

let builder = db.get_database_backend();
db.execute(builder.build(&stmt)).await?;
```

## Select statements

Raw SQL:

```sql
SELECT `baker`.`name` FROM `baker` JOIN `bakery` ON `baker`.`bakery_id` = `bakery`.`id` ORDER BY `baker`.`name` ASC
```

SeaQuery:

If you are only interested in some of the columns, define a struct to hold the query result. It has to derive from the trait `FromQueryResult`.

If all columns are of interest, then the generated `Model` structs (e.g. `baker::Model`) can be used.

The fields of the struct must match the column names of the query result.

```rust, no_run
#[derive(FromQueryResult)]
struct BakerNameResult {
    name: String,
}

...

let column = (baker::Entity, Alias::new("name"));

let mut stmt = Query::select();
stmt.column(column.clone()) // Use `expr_as` instead of `column` if renaming is necessary
    .from(baker::Entity)
    .join(
        JoinType::Join,
        bakery::Entity,
        Expr::tbl(baker::Entity, Alias::new("bakery_id"))
            .equals(bakery::Entity, Alias::new("id")),
    )
    .order_by(column, Order::Asc);

let builder = db.get_database_backend();
let baker = BakerNameResult::find_by_statement(builder.build(&stmt))
    .all(db)
    .await?;

let baker_names = baker.into_iter().map(|b| b.name).collect::<Vec<_>>();

assert_eq!(
    baker_names,
    vec!["Charles", "Frederic", "Jolie", "Madeleine"]
);
```

## Testing and Debugging

It's often useful to check the raw SQL of the SeaQuery-generated statements.

Use `stmt.to_string(query_builder)` to do that.

```rust, no_run
// Check the raw SQL of `stmt` in MySQL syntax
println!({}, stmt.to_string(MysqlQueryBuilder));
```
