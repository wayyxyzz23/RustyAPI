use std::sync::{Arc, Mutex};
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
    description: String,
}

struct AppState {
    items: Arc<Mutex<Vec<Item>>>,
}

async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(*items)
}

async fn create_item(item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let new_id = items.len() as u32 + 1;
    let mut item = item.into_inner();
    item.id = new_id;
    items.push(item);
    HttpResponse::Created().finish()
}

async fn get_item_by_id(info: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id = info.into_inner();
    let items = data.items.lock().unwrap();
    let item = items.iter().find(|&x| x.id == id);
    match item {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn delete_item(info: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id = info.into_inner();
    let mut items = data.items.lock().unwrap();
    if let Some(index) = items.iter().position(|x| x.id == id) {
        items.remove(index);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn update_item(info: web::Path<u32>, item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let id = info.into_inner();
    let item = item.into_inner();
    let mut items = data.items.lock().unwrap();
    let mut updated = false;
    if let Some(it) = items.iter_mut().find(|x| x.id == id) {
        *it = item;
        updated = true;
    }
    if updated {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set in .env file");
    let data = web::Data::new(AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/items", web::get().to(get_items))
            .route("/items", web::post().to(create_item))
            .route("/items/{id}", web::get().to(get_item_by_id))
            .route("/items/{id}", web::delete().to(delete_item))
            .route("/items/{id}", web::put().to(update_item))
    })
    .bind(server_url)?
    .run()
    .await
}