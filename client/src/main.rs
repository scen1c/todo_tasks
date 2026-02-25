use reqwest::Client;
mod cli;



#[tokio::main]
async fn main()  {
    let client = Client::new();
    cli::beginprogram(client).await;
}