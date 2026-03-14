# 🦀 Todo Tasks

A small **client‑server Todo application written in Rust**.

This repository is a learning project demonstrating how to build a full backend stack in Rust:

- REST API with **Axum**
- PostgreSQL database with **SQLx**
- **JWT authentication**
- CLI client using **Reqwest**
- Docker environment for PostgreSQL

The project is structured as a **Rust workspace** containing a server, CLI client, and experimental GUI client.

---

# 🏗 Project Structure

```
todo_tasks
│
├ client/          # CLI client application
├ server/          # Axum REST API server
├ slint-cli/       # GUI client using Slint (work in progress)
│
├ migrations/      # SQL database migrations
├ .sqlx/           # SQLx query metadata
│
├ docker-compose.yml
├ Cargo.toml       # workspace definition
└ README.md
```

---

# ⚙️ Technologies

## Server

- Rust
- Axum
- Tokio
- SQLx
- PostgreSQL
- jsonwebtoken
- dotenvy

## Clients

### CLI Client

- reqwest
- tokio
- serde

### GUI Client (WIP)

- Slint UI framework

---

# 🗄 Database Schema

Defined in:

```
migrations/20260313070204_init_schema.sql
```

### Users

```sql
CREATE TABLE users (
    name TEXT PRIMARY KEY,
    password TEXT NOT NULL
);
```

### Tasks

```sql
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    complited BOOLEAN NOT NULL DEFAULT FALSE,
    user_name TEXT NOT NULL REFERENCES users(name)
);
```

Each task belongs to a user.

---

# 🌐 API

Server runs on

```
http://127.0.0.1:3030
```

## Public endpoints

### Health check

```
GET /alive
```

---

### Register

```
POST /auth/register
```

Body:

```json
{
  "name": "username",
  "password": "password"
}
```

---

### Login

```
POST /auth/login
```

Response:

```json
{
  "access_token": "JWT_TOKEN",
  "token_type": "Bearer"
}
```

---

# 🔐 Authentication

Protected endpoints require header:

```
Authorization: Bearer <token>
```

JWT is generated on login and validated by the server middleware.

---

# 📋 Task Endpoints

### Create task

```
POST /task
```

Body

```json
{
  "title": "learn rust"
}
```

---

### List tasks

```
GET /list
```

Returns all tasks belonging to the authenticated user.

Example:

```json
{
  "tasks": [
    {
      "id": 1,
      "title": "learn rust",
      "complited": false,
      "user_name": "user"
    }
  ]
}
```

---

### Finish task

```
POST /task/finish
```

Marks a task as completed.

---

### Delete task

```
POST /task/delete
```

Removes a task from the database.

---

# 🐳 Running the Project

## 1 Clone repository

```
git clone https://github.com/scen1c/todo_tasks.git
cd todo_tasks
```

---

## 2 Start PostgreSQL

```
docker compose up -d
```

Services:

| Service | Address |
|--------|--------|
| Postgres | localhost:5432 |
| pgAdmin | http://localhost:5050 |

Database configuration:

```
POSTGRES_USER=postgres
POSTGRES_PASSWORD=1234
POSTGRES_DB=tasks_db
```

---

## 3 Environment variables

Create file:

```
server/.env
```

Example:

```
DATABASE_URL=postgres://postgres:1234@localhost:5432/tasks_db
SECRET=super_secret_key
```

---

## 4 Run migrations

Install SQLx CLI:

```
cargo install sqlx-cli --no-default-features --features postgres
```

Run migrations:

```
sqlx migrate run
```

---

## 5 Run server

```
cd server
cargo run
```

Server starts on:

```
127.0.0.1:3030
```

---

## 6 Run CLI client

```
cd client
cargo run
```

---

# 💻 CLI Usage

After starting the CLI client you can:

```
1 register
2 login
3 create task
4 list tasks
5 finish task
6 delete task
```

The client communicates with the server using HTTP requests.

---

# 🎯 Learning Goals

This project demonstrates:

- Rust workspaces
- REST API with Axum
- async programming with Tokio
- JWT authentication
- PostgreSQL integration with SQLx
- CLI client interacting with an API
- Dockerized development environment

---

# 🚧 Future Improvements

Possible improvements for the project:

- use `user_id` instead of `user_name`
- finish tasks by `id`
- add task update endpoint
- add pagination for tasks
- improve error handling
- integration tests
- better CLI UX
- finish Slint GUI client

---

# 👨‍💻 Author

GitHub

```
https://github.com/scen1c
```

---

⭐ If you like the project — give it a star

