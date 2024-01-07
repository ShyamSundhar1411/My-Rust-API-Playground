use chrono::prelude::{DateTime, Local};
use serde::{Deserialize, Serialize};
#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct ToDo{
    pub id: u64,
    pub title: String,
    pub description: String,
    pub is_done: Option<bool>,
    pub created_at: Option<DateTime<Local>>,
}

impl ToDo{
    #[allow(dead_code)]
    pub fn new(id: u64, title: String, description: String) -> Self{
        Self { id: id, title: title, description: description, is_done: Some(false), created_at: Some(Local::now()) }
    }
}


