mod todo;
use log;
use actix_web::{App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize env_logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let todo_service = todo::services::ToDoService::new();

    match HttpServer::new(move || {
        App::new()
            .app_data(todo_service.clone())
            .configure(todo::routers::configure)
    })
    .bind(("127.0.0.1", 8080)) {
        Ok(server) => {
            log::info!("Server running on port 8080");
            server.run().await
        }
        Err(err) => {
            log::error!("Error: {}", err);
            Err(err)
        }
    }
}
