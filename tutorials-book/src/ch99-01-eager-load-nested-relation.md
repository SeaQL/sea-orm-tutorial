# How to load nested relation eagerly?

Say, we want to eagerly select a nested relation. Such as select `film`, `film_actor` and `actor` at once.

![Sakila ER Diagram](https://camo.githubusercontent.com/a5414b9a1f6336e2bbae5bbf358bd3df76627dc9e73d4800ef5822b1737e2df2/68747470733a2f2f7777772e6a6f6f712e6f72672f696d672f73616b696c612e706e67)

However, in SeaORM we can only select single related entity. For example, `film` and its related `film_actor` or `film_actor` with its related `actor`.

```rust, no_run
let film_with_film_actor: Vec<(film::Model, Vec<film_actor::Model>)> = Film::find()
    .find_with_related(FilmActor)
    .all(db)
    .await?;
```

```rust, no_run
let film_actor_with_actor: Vec<(film_actor::Model, Vec<actor::Model>)> = FilmActor::find()
    .find_with_related(Actor)
    .all(db)
    .await?;
```

Why is that?

Since Rust is a compiled language that means every type is known at compile-time. Selecting nested relation require a nested struct to store it. Designing a "dynamic type system" in Rust is no easy task and it involve balancing the trade off between the memory and the CPU cycles. The blog post, "[Q01 Why SeaORM does not nest objects for parent-child relation?](https://www.sea-ql.org/blog/2022-05-14-faq-01/)", explains it in details.

If you are building a web API that perform selecting nested relation extensively. Consider serving a GraphQL server instead. [seaography](https://github.com/SeaQL/seaography) is a GraphQL framework for building GraphQL resolvers using SeaORM entities. With GraphQL resolver in place select above nested relation, `film`, `film_actor` and `actor`, is straightforward and easy. Check "[Getting Started with Seaography](https://www.sea-ql.org/blog/2022-09-27-getting-started-with-seaography/#query-data-via-graphql)" to learn more.

## Related Discussions
- [How to Eager Load more than 1 relationship (SeaQL/sea-orm#1114)](https://github.com/SeaQL/sea-orm/discussions/1114)
- [relation access (SeaQL/sea-orm#1044)](https://github.com/SeaQL/sea-orm/discussions/1044)
- [Eagerly fetch nested relations? (SeaQL/sea-orm#649)](https://github.com/SeaQL/sea-orm/discussions/649)
