use async_graphql::{ComplexObject, Context, Object};
use sea_orm::*;

use crate::entities::{prelude::*, *};

pub(crate) struct QueryRoot;
pub(crate) struct MutationRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello GraphQL".to_owned()
    }

    async fn bakeries(&self, ctx: &Context<'_>) -> Result<Vec<bakery::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Bakery::find().all(db).await
    }

    async fn bakery(&self, ctx: &Context<'_>, id: i32) -> Result<Option<bakery::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Bakery::find_by_id(id).one(db).await
    }

    async fn chefs(&self, ctx: &Context<'_>) -> Result<Vec<chef::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Chef::find().all(db).await
    }

    async fn chef(&self, ctx: &Context<'_>, id: i32) -> Result<Option<chef::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Chef::find_by_id(id).one(db).await
    }
}

#[ComplexObject]
impl bakery::Model {
    async fn chefs(&self, ctx: &Context<'_>) -> Result<Vec<chef::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Chef).all(db).await
    }
}

#[ComplexObject]
impl chef::Model {
    async fn bakery(&self, ctx: &Context<'_>) -> Result<bakery::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Bakery).one(db).await.map(|b| b.unwrap())
    }
}

#[Object]
impl MutationRoot {
    async fn add_bakery(&self, ctx: &Context<'_>, name: String) -> Result<bakery::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Bakery::insert(bakery::ActiveModel {
            name: ActiveValue::Set(name),
            profit_margin: ActiveValue::Set(0.0),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Bakery::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|b| b.unwrap())
    }

    async fn add_chef(
        &self,
        ctx: &Context<'_>,
        name: String,
        bakery_id: i32,
    ) -> Result<chef::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Chef::insert(chef::ActiveModel {
            name: ActiveValue::Set(name),
            bakery_id: ActiveValue::Set(bakery_id),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Chef::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|b| b.unwrap())
    }
}
