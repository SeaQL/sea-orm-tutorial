# Reading User Input

Rust standard library provides an easy way of reading from and writing to the command-line commonly known as `stdout`. First, create a file in the `src` folder called `user_input.rs`.

`File: src/user_input.rs`

```rust,no_run,noplayground
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

```

`read_line()` is responsible for reading `stdout` for the user input and returning the user input as a String.  It always clears the terminal  using `utils::clear_terminal();` before the next input, clears the buffer to prevent stale commands using `buffer.clear()`, lists the list of fruits that the user can add and formats the TODOs printing the sorted TODO list and a set of commands that the user can input to interact with the client.

#### User Input Handler

To handle the input create a file in the `src` directory called `handler.rs`

`File: src/handler.rs`

```rust,no_run,noplayground
use crate::{
    convert_case, done, edit, get_fruits, load_sqlite_cache, loading, read_line, split_words,
    store, synching, undo, update_remote_storage, MemDB,
};
use std::io;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

pub async fn input_handler(db: &DatabaseConnection) -> anyhow::Result<()> {
    let mut username_buffer = String::default();
    println!("What is Your Username...",);
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut username_buffer)?;
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

```

The code block above is nested and there are comments to help understanding it. Simply, it:

- reads the `username`
- looks up the `username` from the remote PostgreSQL database
- Loads the local TODO list cache from the local SQLite database
- Stores the loaded local TODO list cache into `MemDB` in-memory database
- reads `stdin` for user input into a `buffer`
- splits the buffer into individual constituents and stores them in an array
- reads the first index of the array to get the command
- performs conditional operations on the command and performs the necessary database operations
- If the command it not available it exits the program
- If the fruit provided is not available, it clears the buffer and reads `stdin` again
- if the command is `EXIT` , it syncs the local SQLite cache with the remote PostgreSQL database and exits.

Lastly, import the modules into `src/main.rs`

`File: src/main.rs`

```rust,no_run,noplayground
  mod common;
  mod db_ops;
+ mod handler;
  mod todo_list_table;
+ mod user_input;
  mod utils;

  pub use common::*;
  pub use db_ops::*;
+ pub use handler::*;
  pub use todo_list_table::prelude::*;
+ pub use user_input::*;
  pub use utils::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let db = database_config().await?;
    create_todo_table(&db).await?;

+   input_handler(&db).await?;

    Ok(())
}

```

#### Running the Client and Server

Running both the `todo-server` in the `TODO-Server` directory  prints

```sh
$ ../target/debug/todo-server
`CREATE TABLE fruits` "Operation Successful"
`CREATE TABLE todos` "Operation Successful"
Listening on 127.0.0.1:8080

```

Running the `todo-client` in the current directory prints.

```sh
$ Running `target/debug/todo_client`
`CREATE TABLE todo_list` "Operation Successful"
What is Your Username...
```

**Enter a username like `user001`**

This creates a new user in the PostgreSQL database since the user currently does not exist. Querying the PostgreSQL database prints

```sh
fruits_market=# SELECT * FROM todos;
 todo_id | username | todo_list 
---------+----------+-----------
       2 | user001  | 
(1 row)
```

The client then prints a list of fruits, commands and a TODO section:

```sh
+--------------------------+
+ COMMANDS                 +
+                          +
→   ADD                    +
→   DONE                   +
→   UNDO                   +
→   EDIT                   +
→   EXIT                   +
+                          +
+--------------------------+
No.| FRUITS AVAILABLE
----------------
 1 | Apple     
 2 | Orange    
 3 | Mango     
 4 | Pineapple 
--------------------------------------------




Oh My! There are no TODOs
Enter a fruit that is available.

```

**Adding a fruit, like `ADD 5kg Apple` prints:**

```sh
+--------------------------+
+ COMMANDS                 +
+                          +
→   ADD                    +
→   DONE                   +
→   UNDO                   +
→   EDIT                   +
→   EXIT                   +
+                          +
+--------------------------+
No.| FRUITS AVAILABLE
----------------
 1 | Apple     
 2 | Orange    
 3 | Mango     
 4 | Pineapple 
--------------------------------------------




QUANTITY | NOT DONE  
----------------
     5kg | Apple     
----------------

----------------
Bummer :( You Have Not Completed Any TODOs!
----------------


Enter a fruit that is available.

```

A `NOT DONE` table is added and below that the statement `Bummer :( You Have Not Completed Any TODOs!` is printed showing that we have `TODOs` that are not done yet.

**Add another fruit like `ADD 1kg OraNGe` will print:**

```sh
+--------------------------+
+ COMMANDS                 +
+                          +
→   ADD                    +
→   DONE                   +
→   UNDO                   +
→   EDIT                   +
→   EXIT                   +
+                          +
+--------------------------+
No.| FRUITS AVAILABLE
----------------
 1 | Apple     
 2 | Orange    
 3 | Mango     
 4 | Pineapple 
--------------------------------------------




QUANTITY | NOT DONE  
----------------
     5kg | Apple     
     1kg | Orange    
----------------

----------------
Bummer :( You Have Not Completed Any TODOs!
----------------


Enter a fruit that is available.

```

Here, even though the fruit `Orange` is typed as `OraNGe`, it is still added since we handle this in the code using `convert_case()` function.

**Now, edit the orange from `1Kg ` to `3kg` with `EDIT 3kg Orange`**. This prints:

```sh
+--------------------------+
+ COMMANDS                 +
+                          +
→   ADD                    +
→   DONE                   +
→   UNDO                   +
→   EDIT                   +
→   EXIT                   +
+                          +
+--------------------------+
No.| FRUITS AVAILABLE
----------------
 1 | Apple     
 2 | Orange    
 3 | Mango     
 4 | Pineapple 
--------------------------------------------




QUANTITY | NOT DONE  
----------------
     5kg | Apple     
     3kg | Orange    
----------------

----------------
Bummer :( You Have Not Completed Any TODOs!
----------------


Enter a fruit that is available.

```

**Next, mark the `Apple` TODO as `done` using `DONE apple`**. This prints:

```sh
+--------------------------+
+ COMMANDS                 +
+                          +
→   ADD                    +
→   DONE                   +
→   UNDO                   +
→   EDIT                   +
→   EXIT                   +
+                          +
+--------------------------+
No.| FRUITS AVAILABLE
----------------
 1 | Apple     
 2 | Orange    
 3 | Mango     
 4 | Pineapple 
--------------------------------------------




QUANTITY | NOT DONE  
----------------
     3kg | Orange    
----------------

QUANTITY | DONE TODOS
----------------
     5kg | Apple     
----------------

Enter a fruit that is available.

```

A `DONE TODOS` table is created with the `Apple` as a member.

**Next, mark the `Apple` as undone with `UNDO Apple`.** This prints:

```sh
+--------------------------+
+ COMMANDS                 +
+                          +
→   ADD                    +
→   DONE                   +
→   UNDO                   +
→   EDIT                   +
→   EXIT                   +
+                          +
+--------------------------+
No.| FRUITS AVAILABLE
----------------
 1 | Apple     
 2 | Orange    
 3 | Mango     
 4 | Pineapple 
--------------------------------------------




QUANTITY | NOT DONE  
----------------
     5kg | Apple     
     3kg | Orange    
----------------

----------------
Bummer :( You Have Not Completed Any TODOs!
----------------


Enter a fruit that is available.


```

The `Apple` is moved back to the `NOT DONE` table and since there are no DONE TODOs, the `DONE TODO` table is replaced by `Bummer :( You Have Not Completed Any TODOs!` .

Next, complete all TODOs by marking both the `Orange` and `Apple` as done with:

	1. `DONE Apple`
	1. `DONE orange`

This prints:

```sh
+--------------------------+
+ COMMANDS                 +
+                          +
→   ADD                    +
→   DONE                   +
→   UNDO                   +
→   EDIT                   +
→   EXIT                   +
+                          +
+--------------------------+
No.| FRUITS AVAILABLE
----------------
 1 | Apple     
 2 | Orange    
 3 | Mango     
 4 | Pineapple 
--------------------------------------------




Wohooo! All TODOs are Completed.
QUANTITY | DONE TODOS
----------------
     5kg | Apple     
     3kg | Orange    
----------------

Enter a fruit that is available.



```

All TODOs are moved to the `DONE TODOS` table and the `NOT DONE` table is replaced by `Wohooo! All TODOs are Completed.` since all `TODOs` are done. This proves that our logic works. 

Lastly, exit the `todo-client` gracefully with the command `EXIT`. This syncs the in-memory database to the remote PostgreSQL server and then exits the program. It prints:

```sh
SYNCING TO SERVER...
SYNCED SUCCESSFULLY.
Bye! :)
```

The state of the SQLite cache is:

```sql
sqlite> SELECT * FROM todo_list ;
1|Apple|5kg|1
2|Orange|3kg|1
sqlite> 

```

The state of the PostgreSQL server is:

```sql
fruits_market=# SELECT * FROM todos;
 todo_id | username |                                                                        todo_list                                                                         
---------+----------+----------------------------------------------------------------------------------------------------------------------------------------------------------
       2 | user001  | {"queued":[],"completed":[{"todo_id":2,"todo_name":"Orange","quantity":"3kg","status":1},{"todo_id":1,"todo_name":"Apple","quantity":"5kg","status":1}]}
(1 row)

```

This shows that the TODO list has been successfully synced to remote storage. Running the client again with the same username `user001` should print the `DONE TODOS` from the persisted SQLite cache:

```sh
+--------------------------+
+ COMMANDS                 +
+                          +
→   ADD                    +
→   DONE                   +
→   UNDO                   +
→   EDIT                   +
→   EXIT                   +
+                          +
+--------------------------+
No.| FRUITS AVAILABLE
----------------
 1 | Apple     
 2 | Orange    
 3 | Mango     
 4 | Pineapple 
--------------------------------------------




Wohooo! All TODOs are Completed.
QUANTITY | DONE TODOS
----------------
     5kg | Apple     
     3kg | Orange    
----------------

Enter a fruit that is available.


```



All the source code for the program can be found at [https://github.com/SeaQL/sea-orm-tutorial/tree/master/todo-app](https://github.com/SeaQL/sea-orm-tutorial/tree/master/todo-app).

That's it for this tutorial. :)
