use sqlx::PgPool;
use serde::{Deserialize, Serialize};
#[derive(Clone,Serialize)]
struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
pub async fn create_task(pool: &PgPool, title: &str) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO tasks (title) VALUES ($1)";
    sqlx::query(query).bind(&title).execute(pool).await?;

    Ok(())
}

pub async fn list_tasks(pool: &PgPool) -> Result<Vec<(i32, String, bool)>, sqlx::Error> {
    let rows = sqlx::query!("SELECT id, title, completed FROM tasks").fetch_all(pool).await?;

    let tasks: Vec<(i32, String, bool)> = rows.into_iter().map(|r| (r.id, r.title, r.completed)).collect();

    Ok(tasks)
}

pub async fn finish_task(pool: &PgPool, title: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE tasks SET completed = true WHERE title = $1", title).execute(pool).await?;

    Ok(())

}