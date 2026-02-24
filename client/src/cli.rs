use std::io::{self, Write};
use reqwest::Client;
use serde::{Serialize, Deserialize};


#[derive(serde::Serialize)]
struct RegisterRequest {
    name: String,
    password: String,
}
struct LoginRequest {
    name: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    access_token: String,
    token_type: String,
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub async fn beginprogram(client: Client) {
    loop {
    println!("Hello user! Welcome to todo task manager!");
    let answer = read_line("Do u have account?(y/n): ");
    let answer = answer.trim(); 
    match answer {
        "y" => login_cli().await,
        "n" => regist_cli(client.clone()).await,
        _ => println!("Choose only y or n") 
    }
    }
}


async fn login_cli() {

}

pub async fn regist_cli(client: Client) {
    println!("Lets create for u account!");
    let name = read_line("Whats ur name?: ");
    let password = read_line("Enter ur password: ");
    let name = name.trim();
    let password = password.trim();
    let request = RegisterRequest {
        name: name.to_string(),
        password: password.to_string(),
    };
    let requesttosev = client.post("http://127.0.0.1:3030/auth/register").json(&request).send().await;
    match requesttosev {
        Ok(resp) => {
            println!("Ur account created into DB!");
        }
        Err(err) => {
            println!("Request failed: {}", err);
        }
    }
}