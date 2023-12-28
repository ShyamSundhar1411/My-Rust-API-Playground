use actix_web::{web,HttpResponse};
use crate::todo::services::ToDoService;

async fn get_todos(todo_service: web::Data<ToDoService>) -> HttpResponse {
    let todos = todo_service.get_todos();
    HttpResponse::Ok().json(todos)
}