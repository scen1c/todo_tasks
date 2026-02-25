use axum::{
    Json, Router, async_trait, extract::{FromRequestParts, State}, 
    http::{StatusCode, request::Parts}, 
    response::IntoResponse, routing::{get, post}
};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use time::{Duration, OffsetDateTime};



mod core_functions;
use core_functions as cf;



#[derive(Clone)]
struct AppState {
    pool: PgPool,
    jwt_secret: String
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    name: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    name: String,
    password: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    access_token: String,
    token_type: &'static str,
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64
}

#[derive(Debug, Serialize)]
struct OkResponse {
    ok: bool,
}
#[derive(Debug, Deserialize)]
struct TaskRequest {
    title: String
}
#[derive(Debug, Deserialize)]
struct FinishTaskRequest {
    title: String
}


#[derive(Debug, Serialize, Clone)]
struct ListTaskResponse {
    tasks: Vec<cf::Task>
}

fn make_jwt(username: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp: i64 = (OffsetDateTime::now_utc() + Duration::minutes(30)).unix_timestamp();

    let claims = Claims {
        sub: username.to_string(),
        exp,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())?;

    Ok(data.claims)
}

struct UserAuth {
    name: String
}
#[async_trait]
impl FromRequestParts<AppState> for UserAuth {
    type Rejection = (StatusCode, Json<serde_json::Value>);
    
    
    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> { 
    
    let auth = parts
        .headers 
        .get(axum::http::header::AUTHORIZATION) 
        .and_then(|v| v.to_str().ok()) 
        .ok_or_else(|| { (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Missing Authorization header"}))) })?;
    
    let token = auth
        .strip_prefix("Bearer ")
        .ok_or_else(|| { (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Use: Authorization: Bearer <token>"}))) })?; 
    
    let claims = verify_jwt(token, &state.jwt_secret)
        .map_err(|_| { (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid or expired token"}))) })?; 
    
    Ok(UserAuth { name: claims.sub }) }

}



#[tokio::main]
async fn main() {

    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPool::connect(&db_url).await.unwrap();
    let jwt_secret = std::env::var("SECRET").unwrap();
    let state = AppState {pool, jwt_secret};
    let app = Router::new()
        .route("/alive", get(alive))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/task", post(create_task_ser))
        .route("/list", get(list_task_ser))
        .route("/task/finish", post(finish_task_ser))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
}


async fn alive() -> &'static str {
    "Alive"
}

async fn register(State(state): State<AppState>, Json(req): Json<RegisterRequest>) -> impl IntoResponse {
    if req.name.is_empty() || req.password.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "name and password required"}))).into_response()
    }
    let list = cf::list_users(&state.pool).await.unwrap();
    let check = list
        .iter()
        .any(|a| a.name == req.name);
    if check {
        println!("Error of creating user {} cuz alrdy exist", &req.name);
        return (StatusCode::CONFLICT, Json(serde_json::json!({"error": "User already exist!"}))).into_response()
    }
    
    let result = cf::create_user(&state.pool, &req.name, &req.password).await;
    match result  {
        Ok(_) => {
            println!("Added new user into DB");
            return (StatusCode::CREATED, Json(serde_json::json!({"Created": "User created"}))).into_response()
        },
        Err(_) => {
            println!("Error to create user");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "DB error"}))).into_response()
        }
    }

}

async fn login(State(state): State<AppState>, Json(log): Json<LoginRequest>) -> impl IntoResponse {
    if log.name.is_empty() || log.password.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "name and password required"}))).into_response();
    }

    let result = cf::login_in(&state.pool, &log.name, &log.password).await;
    let ok = match result  {
        Ok(authcorrect) => {
            println!("Logged into account {}", log.name);
            authcorrect
        },
        Err(err) => {
            println!("Error to create user");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("DB error: {err}")})))
                .into_response();
        }
    };
    if !ok {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "bad credentials"})))
            .into_response();
    }


    let token = match make_jwt(&log.name, &state.jwt_secret) {
        Ok(t) => {
            println!("Created jwt to user and returned");
            t
        },
        Err(err) =>{
            println!("Error to create jwt for user! Err: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "token generation failed"}))).into_response()
        }
        

    };

    (StatusCode::OK, Json(LoginResponse {
        access_token: token,
        token_type: "Bearer"
        })).into_response()

}

async fn create_task_ser(State(state): State<AppState>, auth: UserAuth, Json(req): Json<TaskRequest>) -> impl IntoResponse {
    if req.title.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "empty title"}))).into_response()
    }
    let result = cf::create_task(&state.pool, &req.title, &auth.name).await;
    let name = &auth.name;
    if let Err(err) = result {
        println!("Error into DB to create task for {name}. Error {err}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "error to create task into db."})))
            .into_response()
    };
    println!("Send to {} that his task  is created", &auth.name);
    (StatusCode::CREATED, Json(OkResponse { ok: true})).into_response()
}                

async fn list_task_ser(State(state): State<AppState>, auth: UserAuth ) -> impl IntoResponse {
    let result  = cf::list_tasks(&state.pool, &auth.name).await;
    let name = &auth.name;
    if let Err(err) = result {                                                                                
        println!("Error take list of task. User {name}, Error {err}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "error to take list of users's tasks"})))
            .into_response()
    }
    let result = result.unwrap();

    (StatusCode::CREATED, Json(ListTaskResponse {
        tasks: result
    })).into_response()
}   


async fn finish_task_ser(State(state): State<AppState>, auth: UserAuth, Json(req): Json<FinishTaskRequest>) -> impl IntoResponse {
    if req.title.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "empty title"}))).into_response()
    }
    let name = auth.name.clone();
    let result = cf::finish_task(&state.pool, &req.title, &auth.name).await;
    if let Err(err) = result {
        println!("Task not found of user {name}. Err: {err}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "error to finish task into db."})))
            .into_response()
    }
    println!("Send to {} that his task is finished", name);

    (StatusCode::OK, Json(OkResponse { ok: true})).into_response()
}