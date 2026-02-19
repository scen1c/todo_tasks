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
3. Exit");

        let answer = read_line("Choose from 1 to 3: ");

        match answer.as_str() {
            "1" => create_task_cli(pool).await,
            "2" => list_tasks_cli(pool).await,
            "3" => break,
            _ => println!("Error: choose 1..3"),
        }
    }
}

pub async fn create_task_cli(pool: &PgPool) {
    let task = read_line("Create new task to DB: ");

    // create_task у тебя может возвращать Task или (), неважно — главное await
    let _ = ts::create_task(pool, &task).await;

    println!("Created into DB!");
}

pub async fn list_tasks_cli(pool: &PgPool) {
    let tasks = ts::list_tasks(pool).await.unwrap();

    println!("\nID | TITLE | COMPLETED");
    for (id, title, completed) in tasks {
        println!("{id} | {title} | {completed}");
    }
}