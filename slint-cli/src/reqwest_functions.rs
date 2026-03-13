use axum::http::response;
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
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
    pub access_token: String,
}

#[derive(Debug, Serialize)]
struct TaskRequest {
    title: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTaskResponse {
    pub tasks: Vec<Task>
}

#[derive(Debug, Serialize)]
struct FinishTaskRequest {
    title: String
}

#[derive(Debug, Serialize)]
struct DeleteTaskRequest {
    id: i32
}

pub async fn regist_user(client: Client, name: String, password: String) -> Result<String, String> {
    let request = RegisterRequest { name, password };

    let response = client
        .post("http://127.0.0.1:3030/auth/register")
        .json(&request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                Ok("Your account was created successfully".to_string())
            } else {
                Err(format!("Register failed: {}", resp.status()))
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

pub async fn login_user(client: Client, name: String, password: String) -> Result<LoginResponse, String> {
    let request = LoginRequest { name: name.clone(), password };

    let response = client
        .post("http://127.0.0.1:3030/auth/login")
        .json(&request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let data: LoginResponse = resp
                    .json()
                    .await
                    .map_err(|err| format!("Failed to parse login response: {}", err))?;

                Ok(data)
            } else {
                Err(format!("Login failed: {}", resp.status()))
            }
        }
        Err(err) => Err(format!("Request error: {}", err)),
    }
}

pub async fn get_tasks(token: &str) -> Result<ListTaskResponse, reqwest::Error> {
    let client = Client::new();

    let tasks = client
        .get("http://127.0.0.1:3030/list")
        .bearer_auth(token)
        .send()
        .await?
        .json::<ListTaskResponse>()
        .await?;

    Ok(tasks)
}


pub async fn create_task(client: Client, jwt_token_acces: String, title: String) -> Result<bool, String> {
    let body = TaskRequest {
        title
    };
    let response = client
        .post("http://127.0.0.1:3030/task")
        .bearer_auth(&jwt_token_acces)
        .json(&body)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                Ok(true)
            } else {
                Err(format!("Create failed: {}", resp.status()))
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}   

pub async fn delete_task(client: Client, jwt_token_access: String, id: String) -> Result<bool, String> {
    let new_id = id.parse().unwrap();
    let body = DeleteTaskRequest {
        id: new_id
    };
    let response = client
    .post("http://127.0.0.1:3030/task/delete")
    .bearer_auth(&jwt_token_access)
    .json(&body)
    .send()
    .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                Ok(true)
            } else {
                Err(format!("Delete failed: {}", resp.status()))
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

/*pub async fn finish_task_cli(client: Client, jwt: LoginResponse) {
    let title = read_line("Which task didu finish td?: ");
    let body = FinishTaskRequest {
        title
    };
    let response = client
        .post("http://127.0.0.1:3030/task/finish")
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
                println!("Task finished successfully! Code: {text}")
            } else {
                println!("Server returned an err {}", resp.status());
                let text = resp.text().await.unwrap();
                println!("Error body: {}", text)
            }
        },
        Err(err) => {
            println!("Request failed: {}", err)
        }
    }
}

pub async fn delete_task_cli(client: Client, jwt: LoginResponse) {
    let id = read_line("Which task want u delete? Please write id of the task: ");
    let id: i32 = id.parse().unwrap();
    let body = DeleteTaskRequest {
        id
    };
    let response = client
        .post("http://127.0.0.1:3030/task/delete")
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
                println!("Task successfully deleted! Code: {}", text);
                
            } else {
                println!("Server returned an err: {}", resp.status());
                let text = resp.text().await.unwrap();
                println!("Error body: {}", text);
            }
        },
        Err(err) => {
            println!("Request failed : {}", err)
        }
    }
}*/