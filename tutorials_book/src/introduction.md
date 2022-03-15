# Introduction

SeaORM is the most feature rich async ORM for integrating a Rust code base with relational databases aiming to be a write code once and run on any popular Relational Database with current support for MySQL, PostgreSQL, MariaDB and SQLite. The tutorials in this book are a gentle introduction to using the `sea-orm` crate and it's cli tool `sea-orm-cli`.

#### Symbols Used

Some symbols used throughout this book make it easier to visualize changes to a file.

To show added or removed code from files, we will use comments or 

`+` to show added code

`-` to show removed code

`...` is used to show only part of the existing code instead of rewriting already existing code in the examples.

`$ ` shows an operation is done on the console/shell 

`postgres=#` shows a PostgreSQL prompt.

#### Chapters

Each tutorial is contained in it's own chapter and each chapter has subsections that walk you though the steps of each tutorial.

- Chapter 1  - This chapter illustrates doing `Create`, `Read`, `Update` and `Delete` `(CRUD)`operations using a MySQL database.
- Chapter 2 - This chapter simulates a real world application combining an in-memory cache, a local application SQLite cache and a remote HTTP API with a PostgreSQL backend used for persistence of data.

Let's get started.
