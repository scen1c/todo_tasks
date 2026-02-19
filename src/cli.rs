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

pub async fn panel(pool: &PgPool) {
    loop {
        println!("\nHello user, what do u wna choose?
1. Create task
2. List tasks
3. Finish task
3. Exit");

        let answer = read_line("Choose from 1 to 4: ");
        let answer: i8 = answer.parse().unwrap();
        match answer {
            1 => create_task_cli(pool).await,
            2 => list_tasks_cli(pool).await,
            3 => finish_task_cli(pool).await,
            4 => break,
            _ => println!("Error: choose 1..3"),
        }
    }
}

pub async fn create_task_cli(pool: &PgPool) {
    let task = read_line("Create new task to DB: ");
    let _ = ts::create_task(pool, &task).await;

    println!("Created into DB!");
}

pub async fn finish_task_cli(pool: &PgPool) {
    let task = read_line("Which task is completed?: ");
    let tasks =ts::list_tasks(pool).await.unwrap();
    let tasks = tasks.iter().find(|a| a.title == task.clone()).expect("Not found this task!");
    let _ = ts::finish_task(pool, &task).await;

    println!("Task completed into DB!")
}


pub async fn list_tasks_cli(pool: &PgPool) {
    let tasks = ts::list_tasks(pool).await.unwrap();
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