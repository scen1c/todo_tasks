use axum::{
    Router, 
    routing::{get, post}
};
use dotenvy::dotenv;




mod core_functions;

mod handlers;
use handlers as hl;



#[tokio::main]
async fn main() {

    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPool::connect(&db_url).await.unwrap();
    let jwt_secret = std::env::var("SECRET").unwrap();
    let state = hl::AppState {pool, jwt_secret};
    let app = Router::new()
        .route("/alive", get(hl::alive))
        .route("/auth/register", post(hl::register))
        .route("/auth/login", post(hl::login))
        .route("/task", post(hl::create_task_ser))
        .route("/list", get(hl::list_task_ser))
        .route("/task/finish", post(hl::finish_task_ser))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
}
