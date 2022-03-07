use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "fruits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub fruit_id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub datetime_utc: DateTime,
    pub unit_price: i32,
    pub sku: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
