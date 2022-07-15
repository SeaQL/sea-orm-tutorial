# SeaORM Tutorials

This repository contains step-by-step tutorials on how to use SeaORM to do CRUD operations on databases in the Rust Language.

The tutorial is based on a software system for managing a simple database to store information of bakeries.

The tutorials contain the following chapters:

1. [**Bakery Backend**](https://www.sea-ql.org/sea-orm-tutorial/ch01-00-build-backend-getting-started.html) - This chapter covers the basics of using SeaORM to interact with the database (a MySQL database is used for illustration). On top of this backend you can build any interface you need.
2. [**Rocket Integration**](https://www.sea-ql.org/sea-orm-tutorial/ch02-00-integration-with-rocket.html) - This chapter explains how to integrate the SeaORM backend into the Rocket framework to create a web application that provides a web API or even a simple frontend.
3. [**GraphQL Integration**](https://www.sea-ql.org/sea-orm-tutorial/ch03-00-integration-with-graphql.html) - This chapter extends the RESTful API of the web application in Chapter 2 to a GraphQL API with only one endpoint but enhanced flexibility of query.

[![Discord](https://img.shields.io/discord/873880840487206962?label=Discord)](https://discord.com/invite/uCPdDXzbdv)

For additional help on **SeaORM** specific questions, join the support Discord chat.

## Running the tutorials

To run the tutorial code, switch to the directory of the tutorial and run cargo

```sh
# Switch to tutorial directory
$ cd tutorial-folder-name

# Run cargo
$ cargo run
```



## Running the tutorial book

To run the tutorial book as local host first install [mdbook](https://crates.io/crates/mdbook)

```sh
$ cargo install mdbook
```

Then switch to the book folder in the current tutorial directory

```sh
$ cd tutorials-book
```

Then run mdbook

```sh
$ mdbook serve --open
```

This will open the book in your default browser. Alternatively, access it from any browser by visiting [http://localhost:3000](http://localhost:3000)

## License

The information and examples provided in this repository are licensed under **Apache-2.0**

Cheers!
