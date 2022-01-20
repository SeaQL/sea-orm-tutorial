# Installation

Installing SeaORM is easy.

1. Install Rust \- [ https://www.rust-lang.org/tools/install]( https://www.rust-lang.org/tools/install)

2. Install SeaORM-Cli that will help in reading a database schema and generating the relevant `Entity`, `Model` and `Relation` of every table in our selected database (`schema`).

   ```sh
   $ cargo install sea-orm-cli
   ```

   

3. Create a new Rust Cargo project
    ```sh
    $ cargo new SimpleCrud --name simple-crud
    ```

4. Switch to the new cargo project

   ```sh
   $ cd simple-crud
   ```

5. Add SeaORM as a dependency in `Cargo.toml` file

   If you have `cargo edit` installed, run

   ```sh
   $ cargo add sea-orm --no-default-features --features "<DATABASE_DRIVER> <ASYNC_RUNTIME> macros"
   ```

   for our case, we will use Mysql, async-std and Rustls features

   ```sh
   $ cargo add sea-orm --no-default-features --features "runtime-async-std-rustls sqlx-mysql macros" 
   ```

   or if you don't have `cargo edit` installed, you can install it by running

   ````sh
   $ cargo install cargo-edit
   ````

   and then run the previous command `cargo add sea-orm ...`  The `cargo-edit` command is a handy tool that allows you to easily add a rust `crate` to the project and also easily update or upgrade a dependency or dependency tree.

   Add the `anyhow` crate to perform easier error handling and `async-std` crate for async operations

   ```sh
   $ cargo add anyhow
   ```

   ```sh
   $ cargo add async-std --features attributes
   ```

   

   You can also add it `SeaORM`	 manually in the `Cargo.toml` file

   ```toml
   sea-orm = { version = "0.5", features = [ <DATABASE_DRIVER>, <ASYNC_RUNTIME>, "macros" ], default-features = false }
   ```

   The `DATABASE_DRIVER` and `ASYNC_RUNTIME` are described in the `SeaORM Concepts` section of this chapter.

   For this tutorial, we will use `async-std` for the async runtime, `rustls` for secure database connections and `mysql` as the desired database backend. However, even if you choose a different set of features the code stays the same since SeaORM manages the backends.

   So add the following to your `Cargo.toml` file

   ```toml
   sea-orm = { version = "0.5", features = [ "runtime-async-std-rustls", "sqlx-mysql", "macros" ], default-features = false}
   anyhow = "1"
   async-std = "1"
   ```

   

6. Make sure that your database server is running, then login and create a database called `fruit_markets`.

   ```sql
   CREATE DATABASE fruit_markets;
   ```

   

7. Create a new user in the database called `webmaster` and with a password `master_char`

   ```sql
   # Step1: Create a new user
   CREATE USER 'webmaster'@'localhost' IDENTIFIED BY 'master_char';
   
   # Step 2: Allow the user to have Read, Write access to all tables in database `fruit_markets`
   GRANT ALL PRIVILEGES ON fruit_markets . * TO 'webmaster'@'localhost';
   
   # Step 3: Enable the above settings
   FLUSH PRIVILEGES;
   
   # Step 4: Logout of the database
   exit
   ```

We are all set to perform CRUD operations from the MySQL database side.

In the next chapter, we will create simple CRUD operations.

