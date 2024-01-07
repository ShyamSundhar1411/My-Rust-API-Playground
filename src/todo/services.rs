use std::sync::{Mutex, Arc};
use crate::todo::models::ToDo;
#[derive(Clone)]
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
    pub fn get_todo_by_id(&self,id:u64) -> Option<ToDo>{
        let todos = self.todos.lock().unwrap();
        return Some(todos.iter().find(|todo|todo.id == id).unwrap().clone());
    }
}
impl Default for ToDoService{
    fn default() -> Self {
        Self::new()
    }

}