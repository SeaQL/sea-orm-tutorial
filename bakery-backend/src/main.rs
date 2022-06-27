mod entities;
mod migrator;

use entities::{prelude::*, *};
use futures::executor::block_on;
use migrator::Migrator;
use sea_orm::*;
use sea_orm_migration::prelude::*;

const DATABASE_URL: &str = "mysql://root:root@localhost:3306";

#[derive(FromQueryResult)]
struct BakerNameResult {
    name: String,
}

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db_name = "bakeries_db";
    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };
    let schema_manager = SchemaManager::new(db); // To investigate the schema

    Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("bakery").await?);
    assert!(schema_manager.has_table("baker").await?);

    // Insert and Update
    {
        let happy_bakery = bakery::ActiveModel {
            name: ActiveValue::Set("Happy Bakery".to_owned()),
            profit_margin: ActiveValue::Set(0.0),
            ..Default::default()
        };
        let res = Bakery::insert(happy_bakery).exec(db).await?;

        let sad_bakery = bakery::ActiveModel {
            id: ActiveValue::Set(res.last_insert_id),
            name: ActiveValue::Set("Sad Bakery".to_owned()),
            profit_margin: ActiveValue::NotSet,
        };
        sad_bakery.update(db).await?;

        let john = baker::ActiveModel {
            name: ActiveValue::Set("John".to_owned()),
            bakery_id: ActiveValue::Set(res.last_insert_id),
            ..Default::default()
        };
        Baker::insert(john).exec(db).await?;
    }

    // Read
    {
        let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
        assert_eq!(bakeries.len(), 1);

        // Finding by id is built-in
        let sad_bakery: Option<bakery::Model> = Bakery::find_by_id(1).one(db).await?;
        assert_eq!(sad_bakery.unwrap().name, "Sad Bakery");

        // Finding by arbitrary column with `filter()`
        let sad_bakery: Option<bakery::Model> = Bakery::find()
            .filter(bakery::Column::Name.eq("Sad Bakery"))
            .one(db)
            .await?;
        assert_eq!(sad_bakery.unwrap().id, 1);
    }

    // Delete
    {
        let john = baker::ActiveModel {
            id: ActiveValue::Set(1), // The primary must be set
            ..Default::default()
        };
        john.delete(db).await?;

        let sad_bakery = bakery::ActiveModel {
            id: ActiveValue::Set(1), // The primary must be set
            ..Default::default()
        };
        sad_bakery.delete(db).await?;

        let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
        assert!(bakeries.is_empty());
    }

    // Relational Select
    {
        let la_boulangerie = bakery::ActiveModel {
            name: ActiveValue::Set("La Boulangerie".to_owned()),
            profit_margin: ActiveValue::Set(0.0),
            ..Default::default()
        };
        let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;

        for baker_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
            let baker = baker::ActiveModel {
                name: ActiveValue::Set(baker_name.to_owned()),
                bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
                ..Default::default()
            };
            Baker::insert(baker).exec(db).await?;
        }

        // First find *La Boulangerie* as a Model
        let la_boulangerie: bakery::Model = Bakery::find_by_id(bakery_res.last_insert_id)
            .one(db)
            .await?
            .unwrap();

        let bakers: Vec<baker::Model> = la_boulangerie.find_related(Baker).all(db).await?;
        let mut baker_names: Vec<String> = bakers.into_iter().map(|b| b.name).collect();
        baker_names.sort_unstable();

        assert_eq!(baker_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
    }

    // Mock Testing
    {
        let db = &MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![
                // First query result
                vec![bakery::Model {
                    id: 1,
                    name: "Happy Bakery".to_owned(),
                    profit_margin: 0.0,
                }],
                // Second query result
                vec![
                    bakery::Model {
                        id: 1,
                        name: "Happy Bakery".to_owned(),
                        profit_margin: 0.0,
                    },
                    bakery::Model {
                        id: 2,
                        name: "Sad Bakery".to_owned(),
                        profit_margin: 100.0,
                    },
                    bakery::Model {
                        id: 3,
                        name: "La Boulangerie".to_owned(),
                        profit_margin: 17.89,
                    },
                ],
            ])
            .append_query_results(vec![
                // Third query result
                vec![
                    baker::Model {
                        id: 1,
                        name: "Jolie".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                    baker::Model {
                        id: 2,
                        name: "Charles".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                    baker::Model {
                        id: 3,
                        name: "Madeleine".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                    baker::Model {
                        id: 4,
                        name: "Frederic".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                ],
            ])
            .into_connection();

        let happy_bakery: Option<bakery::Model> = Bakery::find().one(db).await?;
        assert_eq!(
            happy_bakery.unwrap(),
            bakery::Model {
                id: 1,
                name: "Happy Bakery".to_owned(),
                profit_margin: 0.0,
            }
        );

        let all_bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
        assert_eq!(
            all_bakeries,
            vec![
                bakery::Model {
                    id: 1,
                    name: "Happy Bakery".to_owned(),
                    profit_margin: 0.0,
                },
                bakery::Model {
                    id: 2,
                    name: "Sad Bakery".to_owned(),
                    profit_margin: 100.0,
                },
                bakery::Model {
                    id: 3,
                    name: "La Boulangerie".to_owned(),
                    profit_margin: 17.89,
                },
            ]
        );

        let la_boulangerie_bakers: Vec<baker::Model> = Baker::find().all(db).await?;
        assert_eq!(
            la_boulangerie_bakers,
            vec![
                baker::Model {
                    id: 1,
                    name: "Jolie".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                baker::Model {
                    id: 2,
                    name: "Charles".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                baker::Model {
                    id: 3,
                    name: "Madeleine".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                baker::Model {
                    id: 4,
                    name: "Frederic".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
            ]
        );
    }

    // SeaQuery insert
    {
        let columns: Vec<Alias> = ["name", "profit_margin"]
            .into_iter()
            .map(Alias::new)
            .collect();

        let mut stmt = Query::insert();
        stmt.into_table(bakery::Entity).columns(columns);

        stmt.values_panic(["SQL Bakery".into(), (-100.0).into()]);

        let builder = db.get_database_backend();
        db.execute(builder.build(&stmt)).await?;
    }

    // SeaQuery select
    {
        let column = (baker::Entity, Alias::new("name"));

        let mut stmt = Query::select();
        stmt.column(column.clone())
            .from(baker::Entity)
            .join(
                JoinType::Join,
                bakery::Entity,
                Expr::tbl(baker::Entity, Alias::new("bakery_id"))
                    .equals(bakery::Entity, Alias::new("id")),
            )
            .order_by(column, Order::Asc);

        let builder = db.get_database_backend();
        let baker = BakerNameResult::find_by_statement(builder.build(&stmt))
            .all(db)
            .await?;

        let baker_names = baker.into_iter().map(|b| b.name).collect::<Vec<_>>();

        assert_eq!(
            baker_names,
            vec!["Charles", "Frederic", "Jolie", "Madeleine"]
        );
    }

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
