mod common;
pub use common::*;
mod db_ops;
pub use db_ops::*;
mod todo_list_table;
pub use todo_list_table::prelude::*;
mod utils;
pub use utils::*;

use async_std::{
    io::{self, stdout, Write},
    net::TcpStream,
    prelude::*,
    sync::Mutex,
};
use std::collections::HashMap;

#[async_std::main]

async fn main() -> anyhow::Result<()> {
    let mut username_buffer = String::default();
    println!("What is Your Username...",);
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut username_buffer).await?;
    let username = username_buffer.trim().to_string();

    let db = database_config().await?;
    create_todo_table(&db).await?;

    let mut fruits_list: Vec<String> = get_fruits(&db).await?;

    let mut buffer = String::new();
    let mut text_buffer = String::new();
    let mut memdb = MemDB::new(HashMap::default());
    utils::loading();
    load_sqlite_cache(&db, &mut memdb).await?;

    let remote_result = get_user_remote_storage(&username).await?;
    if let Some(result_data) = remote_result {
        if result_data == "USER_NOT_FOUND" {
            create_new_user(&username).await?;
        }
    }

    update_remote_storage(&memdb, &username).await?;

    loop {
        read_line(&mut buffer, fruits_list.as_ref(), &memdb).await?;
        buffer = buffer.trim().to_owned();
        let words = split_words(buffer.clone());
        let command = words[0].to_lowercase().to_string();
        let mut quantity: &str = "";
        if command.as_str() == "done" || command.as_str() == "undo" {
            text_buffer = convert_case(&words[1]);
        } else if command.as_str() == "exit" {
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
                            utils::synching();
                            let db_result = store(&db, quantity, &text_buffer).await?;
                            load_sqlite_cache(&db, &mut memdb).await?;
                        }
                    } else if command.as_str() == "edit" {
                        if let Some(mut todo_model) = memdb.lock().await.get_mut(&text_buffer) {
                            if todo_model.status != 1 {
                                utils::synching();
                                edit(&db, todo_model, quantity.to_owned()).await?;
                                todo_model.quantity = quantity.to_owned();
                            }
                        } else {
                            continue;
                        }
                    } else if command.as_str() == "done" {
                        if let Some(mut todo_model) = memdb.lock().await.get_mut(&text_buffer) {
                            if todo_model.status == 0 {
                                utils::synching();
                                let updated_model = done(&db, todo_model).await?;
                                *todo_model = updated_model;
                            }
                            continue;
                        } else {
                            continue;
                        }
                    } else if command.as_str() == "undo" {
                        if let Some(mut todo_model) = memdb.lock().await.get_mut(&text_buffer) {
                            if todo_model.status == 1 {
                                utils::synching();
                                let updated_model = undo(&db, todo_model).await?;
                                *todo_model = updated_model;
                            }
                            continue;
                        } else {
                            continue;
                        }
                    } else if command.as_str() == "exit" {
                        update_remote_storage(&memdb, &username).await?;
                        break;
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

async fn read_line(
    buffer: &mut String,
    fruits_list: &Vec<String>,
    memdb: &MemDB,
    //todo_list: &Vec<String>,
) -> anyhow::Result<String> {
    utils::clear_terminal();
    buffer.clear();
    println!("{NUMBER}| {TITLE:10}");
    println!("----------------");
    for (mut index, item) in fruits_list.iter().enumerate() {
        index += 1;
        println!("{index:2} | {item:10}");
    }
    println!("----------------\n\n");
    println!("+--------------------------+");
    println!("+ {:^5}{:17}+", "COMMANDS", " ");
    println!("+{:26}+", " ");
    println!("→   {ADD_COMMAND:5}{:18}+", " ");
    println!("→   {DONE_COMMAND:23}+");
    println!("→   {UNDO_COMMAND:23}+");
    println!("→   {EDIT_COMMAND:23}+");
    println!("+{:26}+", " ");
    println!("+--------------------------+");

    format_todos(&memdb).await;

    println!("Enter a fruit that is available.",);
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(buffer).await?;

    Ok(buffer.to_owned())
}
