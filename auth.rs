use std::collections::HashMap;
use std::sync::Mutex;
use rocket::{get, post, routes};
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use argon2::{self, Config};
use rand::Rng;
use dotenv::dotenv;
use std::env;

type Db = Mutex<HashMap<String, String>>;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}

#[post("/register", data = "<user>")]
fn register(user: Json<User>, db: &State<Db>) -> Status {
    let mut db = db.lock().expect("db lock.");
    if db.contains_key(&user.username) {
        Status::Conflict
    } else {
        let salt: [u8; 16] = rand::thread_rng().gen();
        let config = Config::default();
        let hash = argon2::hash_encoded(user.password.as_bytes(), &salt, &config).unwrap();
        db.insert(user.username.clone(), hash);
        Status::Created
    }
}

#[post("/login", data = "<user>")]
fn login(user: Json<User>, db: &State<Db>) -> Status {
    let db = db.lock().unwrap();
    match db.get(&user.username) {
        Some(hash) => {
            if argon2::verify_encoded(&hash, user.password.as_bytes()).unwrap() {
                Status::Ok
            } else {
                Status::Forbidden
            }
        }
        None => Status::NotFound,
    }
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the User Authentication Service!"
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    let db: Db = Mutex::new(HashMap::new());

    let _ = rocket::build()
        .manage(db)
        .mount("/", routes![index, register, login])
        .launch()
        .await;
}