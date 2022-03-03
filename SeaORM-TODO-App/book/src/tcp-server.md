# Building The TCP Server

## Install necessary dependencies

   - Switch to the `SeaORM-TODO-App/TODO-Server` directory to build the TCP server

       ```sh
       $ cd TODO-Server
       ```

   - Ensure you have installed Rust programming language [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

   - Ensure you have `sea-orm-cli` installed [https://crates.io/crates/sea-orm-cli](https://crates.io/crates/sea-orm-cli)

   - `async-std` will be used as the async library

       ```sh
       $ cargo add async-std --features attributes
       ```
       
       This adds async-std to `Cargo.toml` file
       

       ```toml
       [package]
       name = "todo-server"
       version = "0.1.0"
       edition = "2021"
        
       [dependencies]
       + async-std = { version = "1.10.0", features = ["attributes"] }
       ```

   - Add `anyhow` crate for error handling

       ```sh
       $ cargo add anyhow
       ```
       
       An entry in the`Cargo.toml` file is added
       

       ```toml
       [package]
       name = "todo-server"
       version = "0.1.0"
       edition = "2021"
        
       [dependencies]
       + anyhow = "1.0.53"
         async-std = { version = "1.10.0", features = ["attributes"] }
       ```

   - Add `sea-orm` with the features to enable sql drivers for PostgreSQL backend 

       ```sh
       $  cargo add sea-orm --no-default-features --features "runtime-async-std-rustls sqlx-postgres macros"
       ```
       
       This adds sea-orm to `Cargo.toml`
       

       ```toml
       [package]
       name = "todo-server"
       version = "0.1.0"
       edition = "2021"
        
       [dependencies]
         anyhow = "1.0.53"
         async-std = { version = "1.10.0", features = ["attributes"] }
       + sea-orm = { version = "0.6.0", features = [
       +     "runtime-async-std-rustls",
       +     "sqlx-postgres",
       +     "macros",
       + ], default-features = false }
       ```
       - Change the main function to async function using async-std
       
       ```rust,no_run,noplayground
       - fn main() {
       -     println!("Hello, world!");
       - }
        
       + #[async_std::main]
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
     SeaORM-TODO-App
      |-- Cargo.toml
      |-- TODO-Client
              |-- src
              |-- Cargo.toml
     +   		|-- .env
      |-- TODO-Server
     ```
     
   - Configure the database environment by editing the `.env` file
   
     File: `SeaORM-TODO-App/TODO-Server/.env`
   
     ```sh
     + DATABASE_URL=postgres://webmaster:master_char@localhost/fruits_market
     ```
   
     
   
   Next, we will create all the required tables and their relationships
