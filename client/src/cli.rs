use std::{array::repeat, io::{self, Write}};
use axum::{Error, http::Response};
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone,Serialize)]
struct Task {
     id: i32,
     title: String,
     completed: bool,
     user_name: String
}

#[derive(serde::Serialize)]
struct RegisterRequest {
    name: String,
    password: String,
}
#[derive(Serialize, Debug)]
struct LoginRequest {
    name: String,
    password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoginResponse {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Serialize)]
struct TaskRequest {
    title: String
}

#[derive(Debug, Clone)]
struct ListTaskResponse {
    tasks: Vec<Task>
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
        "y" => login_cli(client.clone()).await,
        "n" => regist_cli(client.clone()).await,
        _ => println!("Choose only y or n") 
    }
    }
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
            println!("Ur account created into DB! {resp:?}");
        }
        Err(err) => {
            println!("Request failed: {}", err);
        }
    }
}



async fn login_cli(client: Client) {
    let name = read_line("Enter ur name: ");
    let password = read_line("Ener ur password: ");
    let request = LoginRequest {
        name: name.to_string(),
        password: password.to_string(),
    };
    let response = client
        .post("http://127.0.0.1:3030/auth/login")
        .json(&request)
        .send()
        .await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let data: LoginResponse = resp.json().await.unwrap();
                panel(client, data, name).await
            }
        }
        Err(err) => {
            println!("Request error: {}", err)
        }
    }
}

pub async fn panel(client: Client, jwt: LoginResponse, name: String) {
    let a = "-".repeat(45);
    
    loop {
    println!("{a}
Hello {name}!
1 Create task
2 list of tasks
3 finish task
4 Exit
{a}"
);
    let input = read_line("Choose from 1 to 4: ");
    let input: u8 = input
        .trim()
        .parse()
        .unwrap();
    match input {
        1 => create_task_cli(client.clone(), jwt.clone()).await,
        2 => list_task_cli().await,
        3 => finish_task_cli().await,
        4 => break,
        _ => println!("Please choose from 1 to 4!")
    }
}
}


pub async fn create_task_cli(client: Client, jwt: LoginResponse) {
    let title = read_line("Whats a new task u want add to ur list?: ");
    let body = TaskRequest {
        title
    };
    let response = client
        .post("http://127.0.0.1:3030/task")
        .bearer_auth(&jwt.access_token)
        .json(&body)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let text = resp.text()
                    .await
                    .unwrap();
                println!("Task created successfully! Code: {text}")
            } else {
                println!("Server returned err status {}", resp.status());
                let text = resp.text()
                    .await
                    .unwrap();
                println!("Error body: {}", text);
            }
        },
        Err(err) => {
            println!("Request failed: {}", err)
        }
    }

}

pub async fn list_task_cli() {

}

pub async fn finish_task_cli() {

}
