use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use dotenv::dotenv;
use std::env;
use log::info;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|e| {
        panic!("Failed to read SERVER_ADDRESS from environment variables: {}", e)
    });
    info!("Starting server at: {}", &server_address);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(greet))
    })
    .bind(&server_address)
    .unwrap_or_else(|_| panic!("Could not bind server to address {}", &server_address))
    .run()
    .await
}
```
```toml
[dependencies]
actix-web = "4"
dotenv = "0.15.0"
log = "0.4.14"
env_logger = "0.9.0"