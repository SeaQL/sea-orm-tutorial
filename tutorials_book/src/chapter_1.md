# Chapter 1 - Simple CRUD Operations

 In this tutorial, SeaORM is used with `async-std` as the async runtime, `rustls` for database TLS connections and `sqlx-mysql` for the MySQL database backend.

### Installation of dependencies and tools

1. Install SeaORM-Cli that will help in reading a database schema and generating the relevant `Entity`, `Model` and `Relation` of every table in our selected database (`schema`).

   ```sh
   $ cargo install sea-orm-cli
   ```

2. Create a new Rust Cargo project
    ```sh
    $ cargo new SimpleCrud --name simple-crud
    ```

3. Switch to the new cargo project

   ```sh
   $ cd simple-crud
   ```

4. Add SeaORM as a dependency in `Cargo.toml` file

   If you have `cargo edit` installed, run

   ```sh
   $ cargo add sea-orm --no-default-features --features "runtime-async-std-rustls sqlx-mysql macros" 
   ```

   or if you don't have `cargo edit` installed, you can install it by running

   ```sh
   $ cargo install cargo-edit
   ```

5. Add the async runtime

    ```sh
    $ cargo add anyhow
    
    $ cargo add async-std --features attributes
    ```

    You can also add them manually in the `Cargo.toml` file

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

