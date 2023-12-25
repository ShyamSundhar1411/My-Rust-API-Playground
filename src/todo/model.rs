use chrono::prelude::{DateTime, Local};
use serde::{Deserialize, Serialize};
#[derive(Debug,Deserialize, Serialize)]
pub struct ToDo{
    pub id: u64,
    pub title: String,
    pub description: String,
    pub is_done: Option<bool>,
    pub created_at: Option<DateTime<Local>>,
}




