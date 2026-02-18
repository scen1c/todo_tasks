use std::io::{self, Write};
use sqlx::PgPool;
use crate::task_service;

pub fn read_line1(text: &mut String) -> &str {
    io::stdout().flush().unwrap();
    io::stdin().read_line(text).unwrap();
    let text = text.trim();
    text
}

pub async fn create_task_cli(pool: &PgPool) {
    let mut task = String::new();
    
    print!("Create new task to DB: ");
    let task = read_line1(&mut task);
    let result = task_service::create_task(pool, task).await;
    println!("Created into DB!");


}