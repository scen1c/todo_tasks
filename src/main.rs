use axum::{routing::get, Router};
use dotenvy::dotenv;
mod task_service;
mod cli;
use cli as cl;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPool::connect(&db_url).await.unwrap();
    cl::beginprogram(&pool).await;
    let app = Router::new().route("/alive", get(alive));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn alive() -> &'static str {
    "Alive"
}