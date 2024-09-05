# Migration (CLI)

_This and the next sections concern those who are getting a taste of SeaORM by creating a new, toy database schema. If you already have an existing database schema, feel free to skip over to [Section 1.4](ch01-04-entity-generation.md)._

In this section, we define the following simple schema with migrations.

![ER diagram of two entities, Bakery and Chef. Chef has a foreign key referencing Bakery.](./assets/er_diagram.png)

## Initialize using `sea-orm-cli`

For beginners, it is recommended to use `sea-orm-cli` to define and run the migrations.

```sh
# Install `sea-orm-cli`
$ cargo install sea-orm-cli

# List all available migration commands that are supported by `sea-orm-cli`
$ sea-orm-cli migrate -h
```

Initialize the `migration` folder:

```sh
$ sea-orm-cli migrate init

# The folder structure will be as follows:
.
├── Cargo.toml
├── migration
│   ├── Cargo.toml
│   ├── README.md
│   └── src
│       ├── lib.rs
│       ├── m20220101_000001_create_table.rs
│       └── main.rs
└── src
    └── main.rs
```

## Define the migrations

Update the migration files to define the `Bakery` and `Chef` tables:

The filename must follow the format `m<date>_<6-digit-index>_<description>.rs`.

For more information about defining migrations, read the documentation of [`SchemaManager`](https://docs.rs/sea-orm-migration/*/sea_orm_migration/manager/struct.SchemaManager.html).

```diff
- m20220101_000001_create_table.rs
+ m20220101_000001_create_bakery_table.rs
+ m20220101_000002_create_chef_table.rs
```

```rust, no_run
// m20220101_000001_create_bakery_table.rs

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_bakery_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bakery::Table)
                    .col(
                        ColumnDef::new(Bakery::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bakery::Name).string().not_null())
                    .col(ColumnDef::new(Bakery::ProfitMargin).double().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bakery::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Bakery {
    Table,
    Id,
    Name,
    ProfitMargin,
}
```

```rust, no_run
// m20220101_000002_create_chef_table.rs

use sea_orm_migration::prelude::*;

use super::m20220101_000001_create_bakery_table::Bakery;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20220101_000002_create_chef_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Chef table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chef::Table)
                    .col(
                        ColumnDef::new(Chef::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chef::Name).string().not_null())
                    .col(ColumnDef::new(Chef::ContactDetails).json())
                    .col(ColumnDef::new(Chef::BakeryId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chef-bakery_id")
                            .from(Chef::Table, Chef::BakeryId)
                            .to(Bakery::Table, Bakery::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Chef table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chef::Table).to_owned())
            .await
    }
}

// For ease of access
#[derive(Iden)]
pub enum Chef {
    Table,
    Id,
    Name,
    ContactDetails,
    BakeryId,
}
```

```rust, no_run
// migration/src/lib.rs

pub use sea_orm_migration::prelude::*;

// Add each migration file as a module
mod m20220101_000001_create_bakery_table;
mod m20220101_000002_create_chef_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Define the order of migrations.
            Box::new(m20220101_000001_create_bakery_table::Migration),
            Box::new(m20220101_000002_create_chef_table::Migration),
        ]
    }
}
```

**Important**: Make sure the following features are enabled in the `migration` crate. The database driver feature must match the database being used.

```diff
# migration/Cargo.toml

...

[dependencies.sea-orm-migration]
version = "^0.12.0"
features = [
+   "sqlx-mysql",
+   "runtime-async-std-native-tls",
]
```

## Perform the migrations

Perform all the migrations through `sea-orm-cli`:

_Make sure you are running this command at the project root._

```sh
# Change the value of DATABASE_URL according to your database implementation.
# Make sure the database name is also supplied for MySQL or PostgreSQL.
$ DATABASE_URL="mysql://root:root@localhost:3306/bakeries_db" sea-orm-cli migrate refresh
```
