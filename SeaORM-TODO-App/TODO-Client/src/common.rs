use crate::MyTodosModel;
use serde::{Deserialize, Serialize}; // The commands to use to perform CRUD operations on PostgreSQL

// The commands to use to perform CRUD operations on PostgreSQL
#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Store { username: String, todo_list: String },
    UpdateTodoList { username: String, todo_list: String },
    Get(String),
    CreateUser(String),
    ListFruits,
    ListSuppliers,
    DeleteUser(String),
}

//  The structure for a TodoList
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct TodoList {
    pub queued: Vec<MyTodosModel>,
    pub completed: Vec<MyTodosModel>,
}
