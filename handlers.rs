use std::env;
use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
    description: String,
}

static mut ITEMS: Vec<Item> = vec![];

async fn get_items() -> impl Responder {
    let items = unsafe { &ITEMS };
    HttpResponse::Ok().json(items)
}

async fn create_item(item: web::Json<Item>) -> impl Responder {
    let mut item = item.into_inner();
    unsafe {
        let new_id = ITEMS.len() as u32 + 1;
        item.id = new_id;
        ITEMS.push(item);
    }
    HttpResponse::Created().finish()
}

async fn get_item_by_id(info: web::Path<u32>) -> impl Responder {
    let id = info.into_inner();
    let item = unsafe { ITEMS.iter().find(|&x| x.id == id) };
    match item {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn delete_item(info: web::Path<u32>) -> impl Responder {
    let id = info.into_inner();
    let mut index = None;
    unsafe {
        for (i, item) in ITEMS.iter().enumerate() {
            if item.id == id {
                index = Some(i);
                break;
            }
        }
        if let Some(index) = index {
            ITEMS.remove(index);
            HttpResponse::Ok().finish()
        } else {
            HttpResponse::NotFound().finish()
        }
    }
}

async fn update_item(info: web::Path<u32>, item: web::Json<Item>) -> impl Responder {
    let id = info.into_inner();
    let item = item.into_inner();
    let mut updated = false;
    unsafe {
        for it in ITEMS.iter_mut() {
            if it.id == id {
                it.name = item.name.clone();
                it.description = item.description.clone();
                updated = true;
                break;
            }
        }
    }
    if updated {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set in .env file");
    
    HttpServer::new(|| {
        App::new()
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