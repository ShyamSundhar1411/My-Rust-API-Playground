use std::sync::{Mutex, Arc};
use serde_json::json;
use actix_web::{web, HttpResponse, Responder};
use crate::todo::models::{ToDo};
pub struct ToDoService{
    todos: Arc<Mutex<Vec<ToDo>>>
}

impl ToDoService{
    pub fn new() -> Self{
        Self { todos: Arc::new(Mutex::new(vec![])), }
    }
    pub fn get_todos(&self) -> Vec<ToDo>{
        let todos = self.todos.lock().unwrap();
        todos.clone()
    }
}
impl Default for ToDoService{
    fn default() -> Self {
        Self::new()
    }

}