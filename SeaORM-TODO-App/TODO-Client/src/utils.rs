use crate::MyTodosModel;
use async_std::sync::Mutex;
use std::collections::HashMap;

pub(crate) const TITLE: &str = "FRUITS AVAILABLE";
pub(crate) const NUMBER: &str = "No.";
pub(crate) const ADD_COMMAND: &str = "ADD";
pub(crate) const DONE_COMMAND: &str = "DONE";
pub(crate) const UNDO_COMMAND: &str = "UNDO";
pub(crate) const EDIT_COMMAND: &str = "EDIT";

const DONE: &str = "DONE TODOS";
const NOT_DONE: &str = "NOT DONE";
const QUANTITY: &str = "QUANTITY";

pub(crate) type MemDB = Mutex<HashMap<String, MyTodosModel>>;

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn synching() {
    clear_terminal();
    println!("SYNCING TO DATABASE...");
}

pub fn loading() {
    clear_terminal();
    println!("LOADING FROM DATABASE...");
}

pub async fn format_todos(todo_models: &MemDB) {
    println!("\n\n\n");
    if todo_models.lock().await.is_empty() {
        println!("Oh My! There are no TODOs");
    } else {
        let mut done = Vec::<MyTodosModel>::default();
        let mut not_done = Vec::<MyTodosModel>::default();

        todo_models.lock().await.iter().for_each(|todo| {
            if todo.1.status == 0 {
                not_done.push(todo.1.to_owned());
            } else {
                done.push(todo.1.to_owned());
            }
        });

        if not_done.is_empty() {
            println!("Wohooo! All TODOs are Completed.")
        } else {
            println!("{QUANTITY:9}| {NOT_DONE:10}");
            println!("----------------");
            not_done.iter().for_each(|todo| {
                println!("{:>8} | {:10}", todo.quantity, todo.todo_name);
            });
            println!("----------------\n");
        }

        if done.is_empty() {
            println!("----------------");
            println!("Bummer :( You Have Not Completed Any TODOs!");
            println!("----------------\n\n");
        } else {
            println!("{QUANTITY:9}| {DONE:10}");
            println!("----------------");
            done.iter().for_each(|todo| {
                println!("{:>8} | {:10}", todo.quantity, todo.todo_name);
            });
            println!("----------------\n");
        }
    }
}

pub fn convert_case(word: &str) -> String {
    let word = word.to_lowercase();
    let mut chars = word
        .chars()
        .map(|character| character.to_string())
        .collect::<Vec<String>>();

    chars[0] = chars[0].to_uppercase().to_string();

    chars.into_iter().collect::<String>()
}

pub fn split_words(user_input: String) -> Vec<String> {
    user_input
        .split(" ")
        .map(|word| word.to_owned())
        .collect::<Vec<String>>()
}
