[dependencies]
actix-web = "4.0"
dotenv = "0.15.0"
serde = "1.0"
serde_json = "1.0"
serde_derive = "1.0"
tokio-diesel = "0.2.0"
diesel = { version = "1.4.5", features = ["postgres", "r2d2", "async", "serde_json"] }
```

```
DATABASE_URL=postgres://username:password@localhost/mydatabase
SERVER_PORT=8080
```

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tokio_diesel::*;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(Serialize, Queryable)]
struct User {
    id: i32,
    name: String,
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let conn = pool.get().expect("Couldn't get db connection from pool");
    let user_data = web::block(move || users.load::<User>(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    web::Json(user_data)
}

async fn create_user(user: web::Json<CreateUser>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::users;

    let conn = pool.get().expect("Couldn't get db connection from pool");

    let new_user = NewUser { name: user.name.clone() };

    let _ = web::block(move || diesel::insert_into(users::table).values(&new_user).execute(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    HttpResponse::Ok().body(format!("User '{}' has been created.", user.name))
}

mod schema {
    table! {
        users (id) {
            id -> Int4,
            name -> Varchar,
        }
    }
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUser {
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    println!("Starting server at: 127.0.0.1:{}", &server_port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(|| async { "Welcome to our API!" }))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
    })
    .bind(format!("127.0.0.1:{}", server_port))?
    .run()
    .await
}