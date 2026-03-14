use axum::extract::{Path, Query, State};
use axum::routing::{get, put};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::calendar::*;
use crate::services::calendar as svc;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/events", get(list_events).post(create_event))
        .route("/api/events/{id}", put(update_event).delete(delete_event))
}

#[derive(Deserialize)]
struct EventRange {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

async fn list_events(
    State(state): State<AppState>,
    Query(range): Query<EventRange>,
) -> Result<Json<Vec<Event>>, AppError> {
    Ok(Json(svc::list_events(&state.pool, range.from, range.to).await?))
}

async fn create_event(
    State(state): State<AppState>,
    Json(input): Json<CreateEvent>,
) -> Result<Json<Event>, AppError> {
    Ok(Json(svc::create_event(&state.pool, input).await?))
}

async fn update_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateEvent>,
) -> Result<Json<Event>, AppError> {
    Ok(Json(svc::update_event(&state.pool, id, input).await?))
}

async fn delete_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    svc::delete_event(&state.pool, id).await
}
