# Introduction

SeaORM is a feature rich async ORM for integrating a Rust code base with relational databases aiming to be a write code once and run on any popular Relational Database with current support for MySQL, PostgreSQL, MariaDB and SQLite. The tutorials in this book are a gentle introduction to using the `sea-orm` crate and its cli tool `sea-orm-cli`.

#### Symbols Used

Some symbols used throughout this book make it easier to visualize changes to a file.

To show added or removed code from files, we will use comments or 

`+` to show added code

`-` to show removed code

`...` is used to show only part of the existing code instead of rewriting already existing code in the examples.

`$ ` shows an operation is done on the console/shell 

#### Chapters

In the first chapter, we will learn how to build a backend application with SeaORM. It will be compatible with different database implementations.

In the subsequent chapters, we will explore the process of building other applications that integrate with a SeaORM-powered backend. In particular, we will be looking at how to build Rocket and GraphQL applications that interact with the backend we created in the first chapter.

Let's get started.
