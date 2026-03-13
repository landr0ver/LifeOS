use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::habit::*;
use crate::services::habits as svc;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/habits", get(list_habits).post(create_habit))
        .route("/api/habits/{id}", put(update_habit).delete(delete_habit))
        .route(
            "/api/habits/{id}/entries",
            post(log_entry).get(get_entries),
        )
}

async fn list_habits(State(state): State<AppState>) -> Result<Json<Vec<Habit>>, AppError> {
    Ok(Json(svc::list_habits(&state.pool).await?))
}

async fn create_habit(
    State(state): State<AppState>,
    Json(input): Json<CreateHabit>,
) -> Result<Json<Habit>, AppError> {
    Ok(Json(svc::create_habit(&state.pool, input).await?))
}

async fn update_habit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateHabit>,
) -> Result<Json<Habit>, AppError> {
    Ok(Json(svc::update_habit(&state.pool, id, input).await?))
}

async fn delete_habit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    svc::delete_habit(&state.pool, id).await
}

async fn log_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateHabitEntry>,
) -> Result<Json<HabitEntry>, AppError> {
    Ok(Json(svc::log_entry(&state.pool, id, input).await?))
}

#[derive(Deserialize)]
struct DateRange {
    from: NaiveDate,
    to: NaiveDate,
}

async fn get_entries(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(range): Query<DateRange>,
) -> Result<Json<Vec<HabitEntry>>, AppError> {
    Ok(Json(svc::get_entries(&state.pool, id, range.from, range.to).await?))
}
