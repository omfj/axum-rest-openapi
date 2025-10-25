CREATE TABLE "users" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    "username" TEXT NOT NULL UNIQUE,
    "email" TEXT NOT NULL UNIQUE,
    "created_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "posts" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    "user_id" INTEGER NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
    "title" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "created_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "sessions" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    "user_id" INTEGER NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
    "session_token" TEXT NOT NULL UNIQUE,
    "expires_at" TEXT NOT NULL
);

CREATE INDEX idx_posts_user_id ON "posts" ("user_id");

CREATE INDEX idx_sessions_user_id ON "sessions" ("user_id");

CREATE INDEX idx_users_username ON "users" ("username");

-- Basic seed data
INSERT INTO
    "users" ("username", "email")
VALUES
    ('jesper', 'jesper@echo.uib.no'),
    ('olem', 'olem@echo.uib.no'),
    ('zeno', 'zeno@echo.uib.no');

INSERT INTO
    "posts" ("user_id", "title", "content")
VALUES
    (1, 'Hello World', 'This is my first post!'),
    (
        2,
        'Axum is great',
        'I love building web apps with Axum.'
    ),
    (
        1,
        'Utoipa for OpenAPI',
        'Documenting APIs made easy with Utoipa.'
    );

INSERT INTO
    "sessions" ("user_id", "session_token", "expires_at")
VALUES
    (1, 'token123', datetime('now', '+1 day')),
    (2, 'token456', datetime('now', '+1 day')),
    (3, 'token789', datetime('now', '+1 day'));