# Formating Utilities

To create a better user experience, data will be formatted before it is displayed to the command line on `stdout`.

#### Overview of Code Formatting

Create a `utils.rs` file which will hold the utilities and add the following code blocks:

`File: src/utils.rs`

```rust,no_run,noplayground
use crate::MyTodosModel;
use tokio::sync::Mutex;
use std::collections::HashMap;

pub(crate) const TITLE: &str = "FRUITS AVAILABLE";
pub(crate) const NUMBER: &str = "No.";
pub(crate) const ADD_COMMAND: &str = "ADD";
pub(crate) const DONE_COMMAND: &str = "DONE";
pub(crate) const UNDO_COMMAND: &str = "UNDO";
pub(crate) const EDIT_COMMAND: &str = "EDIT";
pub(crate) const EXIT_COMMAND: &str = "EXIT";


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
pub fn synching_to_server() {
    println!("SYNCING TO SERVER...");
}

pub fn loading() {
    clear_terminal();
    println!("LOADING FROM DATABASE...");
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

```

The `TITLE` and `NUMBER` constants are used to format the headings for the `fruits` table which displays the list of fruits on the command-line interface. The constants `DONE`, `NOT_DONE` and `QUANTITY` are used as the headings of the TODO list.

#### Interaction Commands

To interact with the client, a user will input a command, similar to pressing a button in a GUI or any other GUI event that performs an operation based on user input. The current list of commands are:

The `ADD_COMMAND` constant holds the `ADD` command. This command allows a user to `queue` a task in the TODO list. The format is `ADD  QUANTITY_IN_KG FRUIT_NAME`.

The `DONE_COMMAND` constant holds the `DONE` command. This command allows a user to mark a task as  `completed`  in the TODO list. The format is `DONE  FRUIT_NAME`.

The `UNDO_COMMAND` constant holds the `UNDO` command. This command allows a user to move a completed task back into the `queue` in the TODO list. The format is `UNDO FRUIT_NAME`.

The `EDIT_COMMAND` constant holds the `EDIT` command. This command allows a user to `modify` a task in the TODO list by changing it's `quantity`. The format is `EDIT  QUANTITY_IN_KG FRUIT_NAME`.

The `EXIT_COMMAND` constant holds the `EXIT` command. This command allows a user to `exit`  the client gracefully and sync the local database cache with the remote PostgreSQL server. The format is `EXIT `.



#### Word formating

A number of functions are presented in the code block above:

`clear_terminal()`  is used to clear the terminal using the command line specific flags `\x1B[2J\x1B[1;1H`

`synching()` is used to show that the TODO list is being synced to the local SQLite database cache.

`synching_to_server()`  is used to show that the TODO list is being synced to the remote PostgreSQL database using the TCP API built in the previous chapter.

`loading()` is used to show that information about the user is being fetched from the remote PostgreSQL database.

`convert_case()` is used to format the `fruit` name to `Title Case`, for example, a user can enter a fruit named `Apple` as `apple`, `Apple`, `aPPLe`, `ApplE`, etc... This makes the user experience much smoother.

`split_words()` is used to split the text buffer from the user input into individual parts that correspond with the format specified in the `Commands` like  `COMMAND  QUANTITY_IN_KG FRUIT_NAME`.



#### In-memory Database

Instead of doing database I/O by querying SQLite database every time we need to check the existence of data, we will use an in-memory database described by `MemDB` which contains  a `Mutex<HashMap<String, MyTodosModel>>` scoped to the internals of the crate. This is a `HashMap` indexed using a `String` which is the name of the todo in the `Model` and the value of the indexing key set to the `MyTodosModel`. The HashMap is protected by a `Mutex` for thread-safety.

#### Formatting the TODO List

To format the list of TODOs in local cache and display them to the command-line interface, add the following to the 

`File: src/utils.rs`

```rust,no_run,noplayground
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

```

`format_todos()` functions takes the in-memory database and loops through it, first checking if there are no TODOs  and prints `"Oh My! There are no TODOs"` . If TODOs are found, it iterates through them and sorts the `completed` todos into the `done` Vector declared by `let mut done = Vec::<MyTodosModel>::default();` or the `queued` into the `not_done` declared by `let mut not_done = Vec::<MyTodosModel>::default();` There are no completed TODOs but there are queued ones, it prints `"Bummer :( You Have Not Completed Any TODOs!"` and if there are no queued TODOs but completed ones, it prints `"Wohooo! All TODOs are Completed."`. 

The `MyTodosModel` is the `Model` for the `Entity` table `todo_list` in the local SQLite database cache. 

Import the `utils` module in the `src/main.rs` file

```rust,no_run,noplayground
  mod common;
  mod db_ops;
  mod todo_list_table;
+ mod utils;

  pub use common::*;
  pub use db_ops::*;
  pub use todo_list_table::prelude::*;
+ pub use utils::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let db = database_config().await?;
    create_todo_table(&db).await?;

    Ok(())
}

```

