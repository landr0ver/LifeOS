use axum::extract::State;
use axum::http::{header, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::AppState;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_at: chrono::DateTime<Utc>,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    if req.password != state.auth_password {
        return Err(AppError::Unauthorized);
    }

    let token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::days(30);

    sqlx::query("INSERT INTO sessions (token, expires_at) VALUES ($1, $2)")
        .bind(&token)
        .bind(expires_at)
        .execute(&state.pool)
        .await?;

    Ok(Json(LoginResponse { token, expires_at }))
}

pub async fn logout(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(token) = extract_token(&req) {
        sqlx::query("DELETE FROM sessions WHERE token = $1")
            .bind(token)
            .execute(&state.pool)
            .await?;
    }
    Ok(StatusCode::OK)
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_token(&req).ok_or(AppError::Unauthorized)?;

    let session = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM sessions WHERE token = $1 AND expires_at > now()",
    )
    .bind(token)
    .fetch_one(&state.pool)
    .await?;

    if session == 0 {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(req).await)
}

fn extract_token<B>(req: &Request<B>) -> Option<&str> {
    req.headers()
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}
