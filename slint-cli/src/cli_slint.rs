use std::{cell::Ref, io::{self, Write}};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::rc::Rc;
use crate::{WelcomeWindow, RegisterWindow};
use slint::ComponentHandle;
#[derive(Debug, Clone,Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize)]
struct TaskRequest {
    title: String
}

#[derive(Debug, Clone, Deserialize)]
struct ListTaskResponse {
    tasks: Vec<Task>
}

#[derive(Debug, Serialize)]
struct FinishTaskRequest {
    title: String
}

#[derive(Debug, Serialize)]
struct DeleteTaskRequest {
    id: i32
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn beginprogram(client: Client, app: &WelcomeWindow) {
    let login_weak = app.as_weak();
    let login_client = client.clone();

    app.on_login_clicked(move || {
        let app = login_weak.unwrap();

        let name = app.get_login_text().to_string();
        let password = app.get_password_text().to_string();

        let client = login_client.clone();
        let app_weak_inner = app.as_weak();

        tokio::spawn(async move {
            let result = login_user(client.clone(), name.clone(), password).await;

            match result {
                Ok(data) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        let app = app_weak_inner.unwrap();
                        app.set_status_text("Login successful".into());
                    });
                    let _ = data;
                }
                Err(err) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        let app = app_weak_inner.unwrap();
                        app.set_status_text(err.into());
                    });
                }
            }
        });
    });

    let register_weak = app.as_weak();
    let register_client = client.clone();
    let register_window: Rc<RefCell<Option<RegisterWindow>>> = Rc::new(RefCell::new(None));
    let register_window_clone = register_window.clone();

    app.on_register_clicked(move || {
        let app = register_weak.unwrap();
        open_register_window(register_client.clone(), &app, register_window_clone.clone());
}); 
}

pub fn open_register_window(
    client: Client,
    welcome_app: &WelcomeWindow,
    register_window: Rc<RefCell<Option<RegisterWindow>>>,
) {
    let window = RegisterWindow::new().unwrap();

    setup_register_window_logic(client, &window);

    window.show().unwrap();

    *register_window.borrow_mut() = Some(window);
    
}
pub fn setup_register_window_logic(client: Client, register_app: &RegisterWindow) {
    let register_weak = register_app.as_weak();
    let register_client = client.clone();

    register_app.on_register_clicked(move || {
        let app = register_weak.unwrap();

        let name = app.get_login_text().to_string();
        let password = app.get_password_text().to_string();

        let client = register_client.clone();
        let app_weak_inner = app.as_weak();

        tokio::spawn(async move {
            let result = regist_user(client, name, password).await;

            match result {
                Ok(message) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        let app = app_weak_inner.unwrap();
                        app.set_status_text(message.into());

                        app.hide().unwrap();
                    });
                }
                Err(err) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        let app = app_weak_inner.unwrap();
                        app.set_status_text(err.into());
                    });
                }
            }
        });
    });
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
4 delete task
5 Exit
{a}"
);
    let input = read_line("Choose from 1 to 5: ");
    let input: u8 = input
        .trim()
        .parse()
        .unwrap();
    match input {
        1 => create_task_cli(client.clone(), jwt.clone()).await,
        2 => list_task_cli(client.clone(), jwt.clone()).await,
        3 => finish_task_cli(client.clone(), jwt.clone()).await,
        4 => delete_task_cli(client.clone(), jwt.clone()).await,
        5 => break,
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

pub async fn list_task_cli(client: Client, jwt: LoginResponse) {
    let response = client
        .get("http://127.0.0.1:3030/list")
        .bearer_auth(&jwt.access_token)
        .send()
        .await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let data: ListTaskResponse  = resp.json()
                    .await
                    .expect("Smth happened wrong");

            println!("{:<5} | {:<20} | {:<10}", "ID", "TITLE", "COMPLETED");
            println!("{}", "-".repeat(45));
            for data in &data.tasks {
                println!(
        "{:<5} | {:<20} | {:<10}",
        data.id,
        data.title,
        data.completed
    );
    }
            }
        },

        Err(err) => {
            println!("Request failed: {}", err)
        }
    }
}

pub async fn finish_task_cli(client: Client, jwt: LoginResponse) {
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
}