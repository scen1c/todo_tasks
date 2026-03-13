-- Add migration script here
CREATE TABLE users (
    name TEXT PRIMARY KEY,
    password TEXT NOT NULL
);

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    complited BOOLEAN NOT NULL DEFAULT FALSE,
    user_name TEXT NOT NULL REFERENCES users(name)
);