use sqlx::PgPool;
use serde::{Serialize, de::value::Error};
#[derive(Debug, Clone,Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
#[derive(Debug, Clone,Serialize)]
pub struct User {
    pub name: String,
    pub password: String,
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

pub async fn create_user(pool: &PgPool, name: &str, password: &str) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO users (name, password) VALUES ($1, $2)";
    sqlx::query(query).bind(&name).bind(&password).execute(pool).await?;

    Ok(())
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(User, "SELECT name, password FROM users").fetch_all(pool).await?;

    Ok(users)
}


pub async fn login_in(pool: &PgPool, name: &str, password: &str) -> Result<bool, sqlx::Error> {
    let users = list_users(pool).await?;

    if let Some(user) = users.iter().find(|u| u.name == name) {
        Ok(user.password == password)
    } else {
        Ok(false)
    }
}

