# Introduction

A simple TODO app that demostrates using SeaORM, SQLite and Postgres to build a simple TODO application. TCP connections are used instead of web frameworks for simplicity due to the APIs being available in the standard library, which is mirrored by async-std async library. 

Let's get started.

#### Symbols Used

To show added or removed code from files, we will use comments or 

`+` to show added code

`-` to show removed code

`...` is used to show only part of the existing code instead of rewriting already existing code in the examples.

`$ ` shows an operation is done on the console/shell 

`postgres=#` shows a postgres prompt

This will make it easier to visualize changes to a file

First, install PostgreSQL and SQLite and ensure PostgreSQL server is running.



#### Create a cargo workspace for the server and frontend 

1. Create a new directory `SeaORM-TODO-App`, a `Cargo.toml` file, a `TODO-Server` and a `TODO-Client`. The `TODO-Server` will contain the source code for the TCP server while the `TODO-Client` will contain the source code for the front-end.

   ```sh
   $ mkdir SeaORM-TODO-App
   
   $ cd SeaORM-TODO-App
   
   $ cargo new TODO-Server --name todo-server
   
   $ cargo new TODO-Client --name todo-client
   ```

   Then register the cargo projects with the `cargo workspace` by creating a workspace file in the current directory.

   **File**:*SeaORM-TODO-App/Cargo.toml* file
   
   ```TOML
   [workspace]
   members = [
   	"TODO-Server",
   	"TODO-Client",
   ]
   ```

Next, we will build the TCP server