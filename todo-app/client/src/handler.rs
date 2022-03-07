use crate::{
    convert_case, done, edit, get_fruits, load_sqlite_cache, loading, read_line, split_words,
    store, synching, undo, update_remote_storage, MemDB,
};
use async_std::io;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

pub async fn input_handler(db: &DatabaseConnection) -> anyhow::Result<()> {
    let mut username_buffer = String::default();
    println!("What is Your Username...",);
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut username_buffer).await?;
    let username = username_buffer.trim().to_string();

    let fruits_list: Vec<String> = get_fruits().await?;

    let mut buffer = String::new();
    let mut text_buffer: String;
    let mut memdb = MemDB::new(HashMap::default());
    loading();
    load_sqlite_cache(db, &mut memdb).await?;

    loop {
        read_line(&mut buffer, fruits_list.as_ref(), &memdb).await?;
        buffer = buffer.trim().to_owned();
        let words = split_words(buffer.clone());
        let command = words[0].to_lowercase().to_string();
        let mut quantity: &str = "";
        if command.as_str() == "done" || command.as_str() == "undo" {
            text_buffer = convert_case(&words[1]);
        } else if command.as_str() == "exit" {
            update_remote_storage(&memdb, &username).await?;
            println!("SYNCED SUCCESSFULLY.");
            println!("Bye! :)");
            break;
        } else {
            quantity = &words[1];
            text_buffer = convert_case(&words[2]);
        }

        if !text_buffer.is_empty() {
            match fruits_list.iter().find(|&fruit| *fruit == text_buffer) {
                None => {
                    if !text_buffer.is_empty() {
                        println!("The fruit `{buffer}` is not available.\n",);
                    }
                    continue;
                }
                Some(_) => {
                    if command.as_str() == "add" {
                        if memdb.lock().await.contains_key(&text_buffer) {
                            continue;
                            //TODO
                        } else {
                            synching();
                            store(&db, quantity, &text_buffer).await?;
                            load_sqlite_cache(&db, &mut memdb).await?;
                        }
                    } else if command.as_str() == "edit" {
                        if let Some(mut todo_model) = memdb.lock().await.get_mut(&text_buffer) {
                            if todo_model.status != 1 {
                                synching();
                                edit(&db, todo_model, quantity.to_owned()).await?;
                                todo_model.quantity = quantity.to_owned();
                            }
                        } else {
                            continue;
                        }
                    } else if command.as_str() == "done" {
                        if let Some(todo_model) = memdb.lock().await.get_mut(&text_buffer) {
                            if todo_model.status == 0 {
                                synching();
                                let updated_model = done(&db, todo_model).await?;
                                *todo_model = updated_model;
                            }
                            continue;
                        } else {
                            continue;
                        }
                    } else if command.as_str() == "undo" {
                        if let Some(todo_model) = memdb.lock().await.get_mut(&text_buffer) {
                            if todo_model.status == 1 {
                                synching();
                                let updated_model = undo(&db, todo_model).await?;
                                *todo_model = updated_model;
                            }
                            continue;
                        } else {
                            continue;
                        }
                    } else {
                        dbg!("Unsupported Command");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
