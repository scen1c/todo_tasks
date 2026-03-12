use reqwest::Client;
mod cli_slint;

use cli_slint as cli;
slint::include_modules!();

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app = WelcomeWindow::new().unwrap();

    cli::beginprogram(client, &app);

    app.run().unwrap();

}