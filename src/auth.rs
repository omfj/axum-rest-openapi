#![allow(dead_code)]

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};

use crate::state::AppState;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub expires_at: String,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

/// Represents an authorized userr with a valid session and user information.
#[derive(Debug, Clone)]
pub struct Auth {
    pub session: Session,
    pub user: User,
}

#[derive(Debug)]
pub struct AuthError(String);

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, self.0).into_response()
    }
}

impl FromRequestParts<AppState> for Auth {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session_id = get_bearer_token(parts)
            .ok_or_else(|| AuthError("Missing or invalid Authorization header".to_string()))?;

        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT id, user_id, expires_at
            FROM sessions
            WHERE session_token = ? AND expires_at > datetime('now')
            "#,
            session_id
        )
        .fetch_one(&state.pool)
        .await
        .map_err(|_| AuthError("Invalid or expired session".to_string()))?;

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email
            FROM users
            WHERE id = ?
            "#,
            session.user_id
        )
        .fetch_one(&state.pool)
        .await
        .map_err(|_| AuthError("User not found".to_string()))?;

        Ok(Auth { session, user })
    }
}

pub fn get_bearer_token(parts: &Parts) -> Option<String> {
    let auth_header = parts
        .headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())?;

    auth_header.strip_prefix("Bearer ").map(|t| t.to_string())
}
