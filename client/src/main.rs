use reqwest::Client;
use serde::{Deserialize, Serialize};
mod cli;
#[derive(Serialize)]
struct LoginRequest {
    name: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    access_token: String,
    token_type: String,
}

#[tokio::main]
async fn main()  {
    let client = Client::new();
    cli::beginprogram(client).await;
}