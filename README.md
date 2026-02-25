# 🦀 Todo Tasks — Rust Client-Server Application

> Full client-server Todo application built with Rust, Axum and PostgreSQL  
> Author: **scen1c**

---

# 📌 Project Overview

**Todo Tasks** is a workspace-based Rust project that implements:

- REST API server (Axum)
- PostgreSQL database (SQLx)
- JWT authentication
- CLI client (Reqwest)
- Dockerized database setup

This project was created as a pet-project to deeply understand backend architecture in Rust.

---

# 🏗 Project Structure

```
todo_tasks/
│
├── server/              # Axum REST API
│   ├── src/
│   └── .env
│
├── client/              # CLI client (reqwest)
│   └── src/
│
├── docker-compose.yml   # PostgreSQL + pgAdmin
└── Cargo.toml           # Workspace config
```

---

# 🐳 Full Setup Guide

## 1️⃣ Install Requirements

Make sure you have installed:

- Rust (https://rustup.rs)
- Docker
- Docker Compose

Check versions:

```
rustc --version
docker --version
```

---

## 2️⃣ Start PostgreSQL via Docker

From project root:

```
docker compose up -d
```

This will start:

- PostgreSQL → localhost:5432
- pgAdmin → http://localhost:5050

---

## 3️⃣ Configure Environment Variables

Create a `.env` file inside the `server/` directory.

Example:

```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/todo_db
JWT_SECRET=super_secret_key
```

⚠ Make sure credentials match docker-compose configuration.

---

## 4️⃣ Create Database Tables

You can either:

### Option A: Use pgAdmin

Open http://localhost:5050

Create database: `todo_db`

Create tables manually:

```sql
CREATE TABLE users (
    name TEXT PRIMARY KEY,
    password TEXT NOT NULL
);

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN DEFAULT FALSE,
    user_name TEXT REFERENCES users(name)
);
```

---

## 5️⃣ Run The Server

```
cd server
cargo run
```

Server will run on:

```
http://127.0.0.1:3000
```

Health check:

```
GET http://127.0.0.1:3000/alive
```

---

## 6️⃣ Run The Client

Open a new terminal:

```
cd client
cargo run
```

The CLI will allow you to:

- Register user
- Login
- Create task
- List tasks (if implemented)
- Finish task (if implemented)

---

# 📡 API Documentation

## Health Check

```
GET /alive
```

---

## Register

```
POST /auth/register
Content-Type: application/json

{
  "name": "username",
  "password": "password"
}
```

---

## Login

```
POST /auth/login
Content-Type: application/json

{
  "name": "username",
  "password": "password"
}
```

Response:

```
{
  "token": "JWT_TOKEN"
}
```

---

## Create Task

```
POST /task
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "Learn Rust"
}
```

---

## List Tasks

```
GET /list
Authorization: Bearer <token>
```

---

# 🔐 Authentication

- JWT-based authentication
- Token lifetime: ~30 minutes
- Authorization header required for protected routes

Format:

```
Authorization: Bearer <token>
```

---

# 🧠 Learning Objectives

This project demonstrates:

- Async Rust (Tokio)
- REST API with Axum
- PostgreSQL integration with SQLx
- JWT authentication flow
- Dockerized development
- Workspace architecture
- CLI ↔ API communication

---

# 🚀 Future Improvements

- Password hashing (bcrypt or argon2)
- Refresh tokens
- Proper user_id foreign keys instead of user_name
- Middleware-based authentication layer
- Structured logging (tracing)
- Integration tests
- Full Dockerization (server + db)
- Migrations via sqlx-cli

---

# ⭐ Author

GitHub: https://github.com/scen1c

---

# 📜 License

This project is created for educational purposes.

