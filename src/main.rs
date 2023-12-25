mod todo;
use actix_web::{get,App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    match HttpServer::new(||App::new().service(hello)).bind(("127.0.0.1",8080)){Ok(server) =>{
        println!("Server running on port 8080");
        server.run().await
    }Err(err) => {
        println!("Error: {}", err);
        Err(err)
    }}
}