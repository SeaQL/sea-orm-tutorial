mod entities;
mod migrator;

use entities::{prelude::*, *};
use futures::executor::block_on;
use migrator::Migrator;
use sea_orm::*;
use sea_orm_migration::prelude::*;

const DATABASE_URL: &str = "mysql://root:root@localhost:3306";
const DB_NAME: &str = "bakeries_db";

#[derive(FromQueryResult)]
struct ChefNameResult {
    name: String,
}

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };
    let schema_manager = SchemaManager::new(db); // To investigate the schema

    Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("bakery").await?);
    assert!(schema_manager.has_table("chef").await?);

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

        let john = chef::ActiveModel {
            name: ActiveValue::Set("John".to_owned()),
            bakery_id: ActiveValue::Set(res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(john).exec(db).await?;
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
        let john = chef::ActiveModel {
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
        for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
            let chef = chef::ActiveModel {
                name: ActiveValue::Set(chef_name.to_owned()),
                bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
                ..Default::default()
            };
            Chef::insert(chef).exec(db).await?;
        }

        // First find *La Boulangerie* as a Model
        let la_boulangerie: bakery::Model = Bakery::find_by_id(bakery_res.last_insert_id)
            .one(db)
            .await?
            .unwrap();

        let chefs: Vec<chef::Model> = la_boulangerie.find_related(Chef).all(db).await?;
        let mut chef_names: Vec<String> = chefs.into_iter().map(|b| b.name).collect();
        chef_names.sort_unstable();

        assert_eq!(chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
    }

    // Loader Testing
    {
        // Inserting two bakeries and their chefs
        let la_boulangerie = bakery::ActiveModel {
            name: ActiveValue::Set("La Boulangerie".to_owned()),
            profit_margin: ActiveValue::Set(0.0),
            ..Default::default()
        };
        let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;
        for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
            let chef = chef::ActiveModel {
                name: ActiveValue::Set(chef_name.to_owned()),
                bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
                ..Default::default()
            };
            Chef::insert(chef).exec(db).await?;
        }
        let la_id = bakery_res.last_insert_id;

        let arte_by_padaria = bakery::ActiveModel {
            name: ActiveValue::Set("Arte by Padaria".to_owned()),
            profit_margin: ActiveValue::Set(0.2),
            ..Default::default()
        };
        let bakery_res = Bakery::insert(arte_by_padaria).exec(db).await?;
        for chef_name in ["Brian", "Christine", "Kate", "Samantha"] {
            let chef = chef::ActiveModel {
                name: ActiveValue::Set(chef_name.to_owned()),
                bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
                ..Default::default()
            };
            Chef::insert(chef).exec(db).await?;
        }
        let arte_id = bakery_res.last_insert_id;

        // First find bakeries as Models
        let bakeries: Vec<bakery::Model> = Bakery::find()
            .filter(
                Condition::any()
                    .add(bakery::Column::Id.eq(la_id))
                    .add(bakery::Column::Id.eq(arte_id))
            )
            .all(db)
            .await?;

        // Then use loader to load the chefs in one query.
        let chefs: Vec<Vec<chef::Model>> = bakeries.load_many(Chef, db).await?;
        let mut la_chef_names: Vec<String> = chefs[0].to_owned().into_iter().map(|b| b.name).collect();
        la_chef_names.sort_unstable();
        let mut arte_chef_names: Vec<String> = chefs[1].to_owned().into_iter().map(|b| b.name).collect();
        arte_chef_names.sort_unstable();

        assert_eq!(la_chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
        assert_eq!(arte_chef_names, ["Brian", "Christine", "Kate", "Samantha"]);

        // clean up bakery for next chapters
        let res: DeleteResult = Chef::delete_many()
            .filter(chef::Column::BakeryId.eq(arte_id))
            .exec(db)
            .await?;
        assert_eq!(res.rows_affected, 4);
        let arte_by_padaria = bakery::ActiveModel {
            id: ActiveValue::Set(arte_id), // The primary must be set
            ..Default::default()
        };
        arte_by_padaria.delete(db).await?;
        let res: DeleteResult = Chef::delete_many()
            .filter(chef::Column::BakeryId.eq(la_id))
            .exec(db)
            .await?;
        assert_eq!(res.rows_affected, 4);
        let la_boulangerie = bakery::ActiveModel {
            id: ActiveValue::Set(la_id), // The primary must be set
            ..Default::default()
        };
        la_boulangerie.delete(db).await?;
    }


    // Mock Testing
    {
        let db = &MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results([
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
            .append_query_results([
                // Third query result
                vec![
                    chef::Model {
                        id: 1,
                        name: "Jolie".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                    chef::Model {
                        id: 2,
                        name: "Charles".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                    chef::Model {
                        id: 3,
                        name: "Madeleine".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                    chef::Model {
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

        let la_boulangerie_chefs: Vec<chef::Model> = Chef::find().all(db).await?;
        assert_eq!(
            la_boulangerie_chefs,
            vec![
                chef::Model {
                    id: 1,
                    name: "Jolie".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 2,
                    name: "Charles".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 3,
                    name: "Madeleine".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
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
        let column = (chef::Entity, Alias::new("name"));

        let mut stmt = Query::select();
        stmt.column(column.clone())
            .from(chef::Entity)
            .join(
                JoinType::Join,
                bakery::Entity,
                Expr::col((chef::Entity, Alias::new("bakery_id")))
                    .equals((bakery::Entity, Alias::new("id"))),
            )
            .order_by(column, Order::Asc);

        let builder = db.get_database_backend();
        let chef = ChefNameResult::find_by_statement(builder.build(&stmt))
            .all(db)
            .await?;

        let chef_names = chef.into_iter().map(|b| b.name).collect::<Vec<_>>();

        assert_eq!(
            chef_names,
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
