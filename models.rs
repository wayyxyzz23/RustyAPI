use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::env;

fn load_environment_variables() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PostComment {
    pub id: i32,
    pub content: String,
    pub blog_post_id: i32,
    pub commenter_id: i32,
}

fn main() {
    load_environment_variables();
}