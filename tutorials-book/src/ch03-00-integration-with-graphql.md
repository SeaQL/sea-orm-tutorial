# Chapter 3 - Integration with GraphQL

We've created a web application with Rocket in Chapter 2, but you may notice that the RESTful API of the application lacks flexibility.

For example, a `GET` request to the endpoint `/bakeries`, if successful, always gives us an array of names of the bakeries in the database. This is a toy implementation to demonstrate how things *could* work, but in reality we also need to provide ways for getting other attributes (e.g. *profit_margin*).

If we simply returns everything in a response every time, it may result in the generation and transmission of unnecessary data. If we insist on keeping things small, we'll have to design and model different use cases and create many endpoints to cater for them.

To combat this, [GraphQL](https://graphql.org/), an alternative solution to RESTful API's, provides the flexibility we *(may)* need.

With GraphQL, the user describes the desired data in the **request body**. Then the server **prepares exactly that** and sends it back in the response. As a result, only one endpoint is needed and no extra work is done.

As the experience is greatly enhanced on the client side, the burden of implementing ways to retrieve data flexibly is heavier on the server side. This problem is severe in the world of JavaScript, as quite a lot of boilerplate code is required to implement a GraphQL server there. However, thanks to Rust's powerful type system and macro support, many of GraphQL's features can actually be implemented rather painlessly.

In this chapter, we'll build a Rocket application with GraphQL support powered by [`async_graphql`](https://crates.io/crates/async-graphql). Of course, `SeaORM` will serve as the bridge between the GraphQL resolvers and the database.
