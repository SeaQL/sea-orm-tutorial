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
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
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
