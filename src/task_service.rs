use sqlx::PgPool;
use serde::Serialize;
#[derive(Debug, Clone,Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
pub async fn create_task(pool: &PgPool, title: &str) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO tasks (title) VALUES ($1)";
    sqlx::query(query).bind(&title).execute(pool).await?;

    Ok(())
}

pub async fn list_tasks(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as!(Task, "SELECT id, title, completed FROM tasks").fetch_all(pool).await?;

    Ok(tasks)
}

pub async fn finish_task(pool: &PgPool, title: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE tasks SET completed = true WHERE title = $1", title).execute(pool).await?;

    Ok(())

}