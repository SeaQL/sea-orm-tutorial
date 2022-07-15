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

    async fn bakers(&self, ctx: &Context<'_>) -> Result<Vec<baker::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Baker::find().all(db).await
    }

    async fn baker(&self, ctx: &Context<'_>, id: i32) -> Result<Option<baker::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Baker::find_by_id(id).one(db).await
    }
}

#[ComplexObject]
impl bakery::Model {
    async fn bakers(&self, ctx: &Context<'_>) -> Result<Vec<baker::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Baker).all(db).await
    }
}

#[ComplexObject]
impl baker::Model {
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

    async fn add_baker(
        &self,
        ctx: &Context<'_>,
        name: String,
        bakery_id: i32,
    ) -> Result<baker::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Baker::insert(baker::ActiveModel {
            name: ActiveValue::Set(name),
            bakery_id: ActiveValue::Set(bakery_id),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Baker::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|b| b.unwrap())
    }
}
