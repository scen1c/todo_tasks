use sqlx::PgPool;
use serde::{Serialize};
#[derive(Debug, Clone,Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub user_name: String
}
#[derive(Debug, Clone,Serialize)]

pub struct User {
    pub name: String,
    pub password: String,
}

pub async fn create_task(pool: &PgPool, title: &str, user_name: &str) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO tasks (title, user_name) VALUES ($1, $2)";
    sqlx::query(query).bind(&title).bind(&user_name).execute(pool).await?;

    Ok(())
}

pub async fn list_tasks(pool: &PgPool, name: &str) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as!(Task, "SELECT id, title, completed, user_name FROM tasks WHERE user_name = $1", name).fetch_all(pool).await?;

    Ok(tasks)
}

pub async fn finish_task(pool: &PgPool, title: &str, user_name: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE tasks SET completed = true WHERE title = $1 AND user_name = $2", title, user_name).execute(pool).await?;

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

pub async fn delete_task(pool: &PgPool, id: &i32, user_name: &str) -> Result<(), sqlx::Error>{
    let query = "DELETE FROM tasks WHERE id = $1 and user_name = $2";
    sqlx::query(query).bind(&id).bind(&user_name).execute(pool).await?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use axum::routing::get;
    use sqlx::PgPool;

    static DBURL: &str = "postgres://postgres:1234@localhost:5432/tasks_test";

    async fn get_pool() -> PgPool {
        PgPool::connect(DBURL).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_user() {
        let pool = get_pool().await;

        let name = format!("test_user_{}", rand::random::<u32>());
        let password = "123";

        create_user(&pool, &name, password).await.unwrap();

        let users = list_users(&pool).await.unwrap();

        assert!(users.iter().any(|u| u.name == name));
    }
    #[tokio::test]
    async fn test_create_task() {
        let pool = get_pool().await;
        let title = format!("test_task_{}", rand::random::<u32>());
        let user_name = "test_user";

        create_task(&pool, &title, &user_name).await.unwrap();

        let list = list_tasks(&pool, user_name).await.unwrap();
        assert!(list.iter().any(|a| a.title == title));
    }

    #[tokio::test]
    async fn test_finish_task() {
        let pool = get_pool().await;
        let title = format!("test_task_for_fin_{}", rand::random::<u32>());
        let user_name = "test_user";
        
        create_task(&pool, &title, &user_name).await.unwrap();
        finish_task(&pool, &title, &user_name).await.unwrap();
        let list = list_tasks(&pool, &user_name).await.unwrap();

        let find_task = list.iter().any(|a| a.title == title);
        assert_eq!(find_task, true);

    }
}