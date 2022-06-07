# Migration (API)

If you prefer to set up and run the migrations programmatically, we provide the `Migrator` API for that.

This section covers how to perform migrations without the need to install and use the CLI tool.

## Preparation

Add the cargo dependency `sea-orm-migration`:

```diff

...

[dependencies]
futures = "0.3.21"
sea-orm = { version = "0.8.0", features = [ "sqlx-mysql", "runtime-async-std-native-tls", "macros" ] }
+ sea-orm-migration = "0.8.3"

...

```

Create a module named `migrator`:

```rust, no_run
// src/main.rs

+ mod migrator;

use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

...
```

```rust, no_run
// src/migrator/mod.rs (create new file)

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![]
    }
}
```

## Define the migrations

Define a `Migration` in a file and include it in `migrator/mod.rs`:

The filename must follow the format `m<date>_<6-digit-index>_<description>.rs`.

For more information about defining migrations, read the documentation of [`SchemaManager`](https://docs.rs/sea-orm-migration/0.8.3/sea_orm_migration/manager/struct.SchemaManager.html).

```rust, no_run
// src/migrator/m20220602_000001_create_bakery_table.rs (create new file)

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20220602_000001_create_bakery_table"
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
// src/migrator/m20220602_000002_create_baker_table.rs (create new file)

use sea_orm_migration::prelude::*;

use super::m20220602_000001_create_bakery_table::Bakery;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20220602_000002_create_baker_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Baker table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Baker::Table)
                    .col(
                        ColumnDef::new(Baker::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Baker::Name).string().not_null())
                    .col(ColumnDef::new(Baker::ContactDetails).json())
                    .col(ColumnDef::new(Baker::BakeryId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-baker-bakery_id")
                            .from(Baker::Table, Baker::BakeryId)
                            .to(Bakery::Table, Bakery::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Baker table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Baker::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Baker {
    Table,
    Id,
    Name,
    ContactDetails,
    BakeryId,
}
```

```rust, no_run
// src/migrator/mod.rs

use sea_orm_migration::prelude::*;

+ mod m20220602_000001_create_bakery_table;
+ mod m20220602_000002_create_baker_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
+           Box::new(m20220602_000001_create_bakery_table::Migration),
+           Box::new(m20220602_000002_create_baker_table::Migration),
        ]
    }
}
```

## Perform the migrations

Use the [`MigratorTrait API`](https://docs.rs/sea-orm-migration/0.8.3/sea_orm_migration/migrator/trait.MigratorTrait.html) to perform the migrations. Verify the correctness of the database schema with [`SchemaManager`](https://docs.rs/sea-orm-migration/0.8.3/sea_orm_migration/manager/struct.SchemaManager.html).

```rust, no_run
// src/main.rs

...

+ use sea_orm_migration::prelude::*;

...

async fn run() -> Result<(), DbErr> {

    ...

+   let schema_manager = SchemaManager::new(db); // To investigate the schema

+   Migrator::install(db).await?;
+   assert!(schema_manager.has_table("seaql_migrations").await?);

+   Migrator::refresh(db).await?;
+   assert!(schema_manager.has_table("bakery").await?);
+   assert!(schema_manager.has_table("baker").await?);

    Ok(())
}

...
```
