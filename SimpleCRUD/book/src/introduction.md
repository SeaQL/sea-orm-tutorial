# Introduction

Sea-ORM is an amazing ORM that aims to be a write code once and run on any popular Relational Database with current support for MySQL, PostgreSQL, MariaDB and SQLite.

An ORM, short for Object Relational Mapper, is a programming library to help you  interact with a relational database from an Object-Oriented Programming  (OOP) language.

Tables and columns in a database are mapped to objects and attributes,  while additional methods allow you to load and store data from and to  the database.

Services built in Rust are lightweight (small binary size, low memory usage), safe (with compile-time guarantee), correct (if unit tests are  well-designed), and fast (compile-time optimizations minimize runtime  overhead).

Due to Rust being a static, strongly typed, compiled,  thread-safe, non-garbage-collected, and unconventional object-oriented  language, working with an ORM in Rust is a bit different from other  scripting languages you are already familiar with.

SeaORM tries to help you in reaping the above benefits while avoiding the hiccups when programming in Rust.

### Async Support

Async is a priority for SeaORM and it supports [Tokio](https://crates.io/crates/tokio), [async-std](https://crates.io/crates/async-std) and [Actix](https://crates.io/crates/actix) libraries which are some of the most popular async libraries in the Rust ecosystem, database operations are done through [SQLx](https://crates.io/crates/sqlx) library which implements database drivers for PostgeSQL, MySQL, MariaDB and SQLite in pure Rust.

### Testable Service Oriented ORM

SeaORM offers a uniform API that you can use for mock connections to various database backends to write unit tests for your logic and  build services with speed, supporting  join, filter, sort and pagination.

### Comparison with Diesel

Diesel ORM is the most popular ORM for Rust relational database ecosystem. Diesel shares some features with SeaORM since they are both relational database ORMs, they are schema first and they both support  PostgeSQL, MySQL, MariaDB and SQLite. However, they differ in some features; Diesel is synchronous which SeaORM has supported async from day 1, SeaORM allows for dynamic queries while Diesel is static and Diesel depends on platform specific native database drivers while SeaORM offers drivers in pure Rust.
