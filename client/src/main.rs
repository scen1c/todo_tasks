use reqwest::Client;
use serde::{Deserialize, Serialize};
mod cli;



#[tokio::main]
async fn main()  {
    let client = Client::new();
    cli::beginprogram(client).await;
}