use axum::{
    Json,
    extract::{Path, State},
};

use crate::{auth::Auth, state::AppState};

#[utoipa::path(get, path = "/", responses((status = OK, body = String)))]
pub async fn health() -> &'static str {
    "OK"
}

#[derive(utoipa::ToSchema, sqlx::FromRow, serde::Serialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

#[utoipa::path(
    get,
    path = "/posts",
    responses((status = OK, body = Vec<Post>))
)]
pub async fn list_posts(State(state): State<AppState>) -> Json<Vec<Post>> {
    let posts = sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, content, created_at
        FROM posts
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&state.pool)
    .await
    .expect("Failed to fetch posts");

    Json(posts)
}

#[derive(utoipa::ToSchema, serde::Serialize, serde::Deserialize)]
pub struct CreatePostBody {
    pub title: String,
    pub content: String,
}

#[utoipa::path(
    post,
    path = "/posts",
    request_body = CreatePostBody,
    security(
        ("session" = [])
    ),
    responses(
        (status = CREATED, body = Post),
        (status = UNAUTHORIZED, description = "Unauthorized")
    )
)]
pub async fn create_post(
    State(state): State<AppState>,
    auth: Auth,
    Json(payload): Json<CreatePostBody>,
) -> Json<Post> {
    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (user_id, title, content)
        VALUES (?, ?, ?)
        RETURNING id, title, content, created_at
        "#,
        auth.user.id,
        payload.title,
        payload.content
    )
    .fetch_one(&state.pool)
    .await
    .expect("Failed to create post");

    Json(post)
}

#[utoipa::path(
    get,
    path = "/user/{id}/posts",
    request_body = CreatePostBody,
    responses(
        (status = OK, body = Vec<Post>),
    )
)]
pub async fn list_user_posts(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Json<Vec<Post>> {
    let posts = sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, content, created_at
        FROM posts
        WHERE user_id = ?
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(&state.pool)
    .await
    .expect("Failed to fetch user posts");

    Json(posts)
}
