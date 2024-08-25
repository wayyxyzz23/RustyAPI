use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::env;

fn init_env() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author_id: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub post_id: i32,
    pub author_id: i32,
}

fn main() {
    init_env();
}