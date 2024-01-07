use actix_web::web;
use crate::todo::handlers;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
        .route("", web::get().to(handlers::get_todos))
        .route("/{id}", web::get().to(handlers::get_todo_by_id)),
    );
}



