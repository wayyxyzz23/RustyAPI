use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;

pub async fn establish_connection() -> Pool<Postgres> {
    dotenv().ok(); // Load .env file

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn add_user(pool: &Pool<Postgres>, username: &str, password: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        username,
        password
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_user(pool: &Pool<Postgres>, username: &str) -> Result<String, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT password FROM users WHERE username = $1",
        username
    )
    .fetch_one(pool)
    .await?;

    Ok(row.password)
}