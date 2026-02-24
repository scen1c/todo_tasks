use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router, async_trait
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
    acces_token: String,
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
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
}


async fn alive() -> &'static str {
    "Alive"
}

async fn register(State(state): State<AppState>, Json(req): Json<RegisterRequest>) -> impl IntoResponse {
    if req.name.is_empty() || req.password.is_empty() {
        return StatusCode::BAD_REQUEST
    }
    
    let result = cf::create_user(&state.pool, &req.name, &req.password).await;
    match result  {
        Ok(_) => {
            println!("Added new user into DB");
            StatusCode::CREATED
        },
        Err(_) => {
            println!("Error to create user");
            StatusCode::INTERNAL_SERVER_ERROR
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
            println!("Error to create jwt for user!");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "token generation failed. The code: {err}"}))).into_response()
        }
        

    };

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "access_token": token,
            "token_type": "Bearer"
        })),
    )
        .into_response()

}

