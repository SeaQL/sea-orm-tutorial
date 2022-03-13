# Building The Web Server

### HTTP as the Protocol

The client and server need a structured way to communicate with each other. HTTP will be the protocol chosen for this tutorial using simple `GET` and `POST`.

## Install necessary dependencies

   - Switch to the `todo-app/server` directory to build the Web Server

       ```sh
       $ cd server
       ```

   - Ensure you have installed Rust programming language [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

   - Ensure you have `sea-orm-cli` installed [https://crates.io/crates/sea-orm-cli](https://crates.io/crates/sea-orm-cli)

   - `tokio` will be used as the async library used as it integrates well with `axum` which is the `HTTP framework` used

       ```sh
       $ cargo add tokio --features full
       ```
       
       This adds `tokio` to `Cargo.toml` file
       

       ```toml
       [package]
       name = "server"
       version = "0.1.0"
       edition = "2021"
        
       [dependencies]
       + tokio = { version = "1.17.0", features = ["attributes"] } 
       ```

   - Add `anyhow` crate for error handling, `axum` crate for HTTP handling, `dotenv` for fetching environment variables and `once_cell` to allow global access to the database connection `sea_orm::DatabaseConnection`.

       ```sh
       $ cargo add anyhow axum dotenv once_cell
       ```

       An entry in the `Cargo.toml` file is added
       
       ```toml
       [package]
       name = "server"
       version = "0.1.0"
       edition = "2021"
       
       [dependencies]
       + anyhow = "1.0.53"
       + axum = "1.0.0"
       + dotenv = "0.15.0"
       + once_cell = "1.10.0"
       tokio = { version = "1.17.0", features = ["full"] }
       ```

   - Add `serde` with the features to `derive` 

       ```sh
       $ cargo add serde --features derive
       ```

       This will allow deserialization of `JSON` requests from the client. `serde` is now added to `Cargo.toml` file.

       ```toml
         [package]
         name = "server"
         version = "0.1.0"
         edition = "2021"
       
         [dependencies]
         anyhow = "1.0.53"
         axum = "1.0.0"
         dotenv = "0.15.0"
         once_cell = "1.10.0"
       + serde = { version = "1.0.136", features = ["derive"] }
         tokio = { version = "1.17.0", features = ["full"] }
       
       ```

       

   - Add `sea-orm` with the features to enable sql drivers for PostgreSQL backend 

       ```sh
       $ cargo add sea-orm --no-default-features --features "runtime-tokio-rustls sqlx-postgres macros"
       ```
       
       This adds sea-orm to `Cargo.toml`
       

       ```toml
       [package]
       name = "server"
       version = "0.1.0"
       edition = "2021"
        
       [dependencies]
         anyhow = "1.0.53"
         axum = "1.0.0"
         dotenv = "0.15.0"
         once_cell = "1.10.0"
         serde = { version = "1.0.136", features = ["derive"] }
         tokio = { version = "1.17.0", features = ["full"] }
       
       + sea-orm = { version = "0.6.0", features = [
       +     "runtime-tokio-rustls",
       +     "sqlx-postgres",
       +     "macros",
       + ], default-features = false }
       ```
       Change the main function to async function for integration with `tokio` and using `anyhow` crate to handle and propagate the errors.
       
       ```rust,no_run,noplayground
       - fn main() {
       -     println!("Hello, world!");
       - }
        
       + #[tokio::main]
       + async fn main() -> anyhow::Result<()> {
       +     Ok(())
       + }
       ```

## Creating a new user and database

   - Login to Postgres database and create a new user and database

       ```sh
       $ sudo -u postgres psql postgres
       ```

   - Create a new user in the PostgreSQL prompt.

       ```sh
       postgres=# CREATE ROLE webmaster LOGIN PASSWORD 'master_char';
       ```

   - Create the `fruits_market` database and assign it to the `webmaster` user

       ```sh
       postgres=# CREATE DATABASE fruits_market WITH OWNER = webmaster;
       ```

## Configuring the database environment

   - Create a `.env` file in the workspace directory

     The file structure should look 
     
     ```sh
     todo-app
      |-- Cargo.toml
      |-- server
              |-- src
              |-- Cargo.toml
     +   		|-- .env
      |-- client
     ```
     
   - Configure the database environment by editing the `.env` file
   
     File: `todo-app/server/.env`
   
     ```sh
     + DATABASE_URL=postgres://webmaster:master_char@localhost/fruits_market
     ```
   
     

   Next, we will create all the required tables and their relationships
