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
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    let db = database_config().await?;

    create_todo_table(&db).await?;
    // Get the fruits first
    let get_fruits = Command::ListFruits;
    let serialized_command = bincode::serialize(&get_fruits)?;
    let mut fruits_list: Vec<String>;

    stream.write_all(&serialized_command).await?;

    let mut fruits_buf = vec![0u8; 4096];
    loop {
        let n = stream.read(&mut fruits_buf).await?;
        let rx: Vec<String> = bincode::deserialize(&fruits_buf).unwrap();

        fruits_list = rx;

        if n != 0 {
            break;
        }
    }

    let mut buffer = String::new();
    let mut text_buffer = String::new();
    let mut memdb = MemDB::new(HashMap::default());
    utils::loading();
    load_sqlite_cache(&db, &mut memdb).await?;

    loop {
        read_line(&mut buffer, fruits_list.as_ref(), &memdb).await?;
        buffer = buffer.trim().to_owned();
        let words = split_words(buffer.clone());
        dbg!(&words);
        let command = words[0].to_lowercase().to_string();
        let quantity = &words[1];
        let text_buffer = convert_case(&words[2]);

        if !text_buffer.is_empty() {
            match fruits_list.iter().find(|&fruit| *fruit == text_buffer) {
                None => {
                    if !text_buffer.is_empty() {
                        println!("The fruit `{buffer}` is not available.\n",);
                    }
                    buffer.clear();
                    continue;
                }
                Some(_) => {
                    if command.as_str() == "add" {
                        if memdb.lock().await.contains_key(&text_buffer) {
                            //TODO
                        } else {
                            utils::synching();
                            let db_result = store(&db, quantity, &text_buffer).await?;
                            load_sqlite_cache(&db, &mut memdb).await?;
                        }
                    } else if command.as_str() == "edit" {
                        if let Some(mut todo_model) = memdb.lock().await.get_mut(&text_buffer) {
                            utils::synching();
                            edit(&db, todo_model, quantity.to_owned()).await?;
                            todo_model.quantity = quantity.to_owned();
                        } else {
                            buffer.clear();
                            continue;
                        }
                    } else {
                        dbg!(command);
                        break;
                    }

                    continue;
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
    println!("+ → {ADD_COMMAND:5}{:18}+", " ");
    println!("+ → {REMOVE_COMMAND:23}+");
    println!("+ → {DONE_COMMAND:23}+");
    println!("+ → {UNDO_COMMAND:23}+");
    println!("+ → {EDIT_COMMAND:23}+");
    println!("+{:26}+", " ");
    println!("+--------------------------+");

    format_todos(&memdb).await;

    println!("Enter a fruit that is available.",);
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(buffer).await?;

    Ok(buffer.to_owned())
}
