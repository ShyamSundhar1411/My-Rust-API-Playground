use actix_web::{web,HttpResponse,http::StatusCode};
use crate::todo::services::ToDoService;

pub async fn get_todos(todo_service: web::Data<ToDoService>) -> HttpResponse {
    let todos = todo_service.get_todos();
    HttpResponse::Ok().json(todos)
}

pub async fn get_todo_by_id(todo_service: web::Data<ToDoService>, path: web::Path<u64>) -> HttpResponse{
    let id = path.into_inner();
    match todo_service.get_todo_by_id(id) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::build(StatusCode::NOT_FOUND).body("Todo not found")
    
    }
}