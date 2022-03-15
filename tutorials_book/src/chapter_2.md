# Chapter 2 - A Command-line TODO app

A simple TODO app that demostrates using SeaORM, SQLite and Postgres to build a simple TODO application. This tutorial will simulate building an app with a local SQLite cache and remote storage of the contents of the cache using a HTTP server with a PostgreSQL backend.

First, install PostgreSQL and SQLite and ensure PostgreSQL server is running.

### Initializing the project directory

A cargo workspace make development easier and share the building environment. The `HTTP` TODO client will be called `client` and the `HTTP server` will be called `server`.

#### Initialize the `client` and `server`

Create the workspace directory `todo-app`

```sh
$ mkdir todo-app
```

Then switch to the workspace directory

```sh
$ cd todo-app
```

Create the `client` and `server` projects

```sh
$ cargo new client
```

```sh
$ cargo new server
```

Create a `Cargo.toml` in the root of the workspace directory to register the two projects

`File: todo-app/Cargo.toml`

```toml
[workspace]
members = [
	"client",
	"server",
]
```

Next up is building the Web Server.
