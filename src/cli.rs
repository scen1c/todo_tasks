use std::io::{self, Write};
use sqlx::PgPool;
use crate::task_service as ts;

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub async fn beginprogram(pool: &PgPool) {
    loop{
        let choice = read_line("Hello user! Do u have account?(y/n): ");
        let choice = choice.trim().to_lowercase();
        match choice.as_str() {
            "y" => login_in_cli(pool).await,
            "n" => create_user_cli(pool).await,
            _ => println!("please choose y or n")
            
        }
    }
}

pub async fn panel(pool: &PgPool, user: &str) {
    loop {
        println!("\nHello {user}, what do u wna choose?
1. Create task
2. List tasks
3. Finish task
4. Exit");

        let answer = read_line("Choose from 1 to 4: ");
        let answer: i8 = answer.parse().unwrap();
        match answer {
            1 => create_task_cli(pool, user).await,
            2 => list_tasks_cli(pool, user).await,
            3 => finish_task_cli(pool, user).await,
            4 => break,
            _ => println!("Error: choose 1..3"),
        }
    }
}

pub async fn create_task_cli(pool: &PgPool, name: &str) {
    let task = read_line("Create new task to DB: ");
    let _ = ts::create_task(pool, &task, name).await;

    println!("Created into DB!");
}

pub async fn finish_task_cli(pool: &PgPool, name: &str) {
    let task = read_line("Which task is completed?: ");
    let tasks =ts::list_tasks(pool, name).await.unwrap();
    let tasks = tasks.iter().find(|a| a.title == task.clone()).expect("Not found this task!");
    let _ = ts::finish_task(pool, &task, name).await;

    println!("Task completed into DB!")
}


pub async fn list_tasks_cli(pool: &PgPool, name: &str) {
    let tasks = ts::list_tasks(pool, name).await.unwrap();
    println!("{:<5} | {:<20} | {:<10}", "ID", "TITLE", "COMPLETED");
    println!("{}", "-".repeat(45));
    for task in &tasks {
    println!(
        "{:<5} | {:<20} | {:<10}",
        task.id,
        task.title,
        task.completed
    );
    }
}

pub async fn create_user_cli(pool: &PgPool) {
    let user = read_line("Please write ur name: ");
    let password = read_line("Please create ur password: ");
    let _ = ts::create_user(pool, &user, &password).await;
    
    println!("User was created into db!");
}

pub async fn login_in_cli(pool: &PgPool) {
    let user = read_line("Please write ur name: ");
    let password = read_line("Please write ur password: ");
    let founded = ts::login_in(pool, &user, &password).await;
    match founded {
        Ok(true) => {
            println!("Login into account success!");
            panel(pool, &user).await;
        },
        Ok(false) => println!("Name or password is incorrect"),
        Err(_) => println!("Error of db")
    }
}