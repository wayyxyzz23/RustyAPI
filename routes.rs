use actix_web::{web, App, HttpServer, Responder};
use std::env;
use dotenv::dotenv;

async fn index() -> impl Responder {
    "Welcome to our API!"
}

async fn get_users() -> impl Responder {
    "Here are all the users."
}

async fn get_user_by_id(id: web::Path<u32>) -> impl Responder {
    format!("Fetching user with ID: {}", id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("127.0.0.1:{}", server_port);

    println!("Starting server at: {}", &server_address);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user_by_id))
    })
    .bind(server_address)?
    .run()
    .await
}