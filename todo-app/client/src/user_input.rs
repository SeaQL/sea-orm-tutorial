use crate::{
    format_todos, MemDB, ADD_COMMAND, DONE_COMMAND, EDIT_COMMAND, EXIT_COMMAND, NUMBER, TITLE,
    UNDO_COMMAND,
};
use std::io;

pub async fn read_line(
    buffer: &mut String,
    fruits_list: &Vec<String>,
    memdb: &MemDB,
    //todo_list: &Vec<String>,
) -> anyhow::Result<String> {
    crate::clear_terminal();
    buffer.clear();
    println!("+--------------------------+");
    println!("+ {:^5}{:17}+", "COMMANDS", " ");
    println!("+{:26}+", " ");
    println!("→   {ADD_COMMAND:5}{:18}+", " ");
    println!("→   {DONE_COMMAND:23}+");
    println!("→   {UNDO_COMMAND:23}+");
    println!("→   {EDIT_COMMAND:23}+");
    println!("→   {EXIT_COMMAND:23}+");
    println!("+{:26}+", " ");
    println!("+--------------------------+");

    println!("{NUMBER}| {TITLE:10}");
    println!("----------------");
    for (mut index, item) in fruits_list.iter().enumerate() {
        index += 1;
        println!("{index:2} | {item:10}");
    }
    println!("--------------------------------------------");
    format_todos(&memdb).await;

    println!("Enter a fruit that is available.",);
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(buffer)?;

    Ok(buffer.to_owned())
}
