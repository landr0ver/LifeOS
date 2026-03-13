use axum::extract::{Path, State};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::kanban::*;
use crate::services::kanban as svc;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/boards", get(list_boards).post(create_board))
        .route("/api/boards/{id}", get(get_board).delete(delete_board))
        .route("/api/boards/{id}/columns", post(create_column))
        .route("/api/columns/{id}/cards", post(create_card))
        .route("/api/cards/{id}", put(update_card).delete(delete_card))
}

async fn list_boards(State(state): State<AppState>) -> Result<Json<Vec<Board>>, AppError> {
    Ok(Json(svc::list_boards(&state.pool).await?))
}

async fn create_board(
    State(state): State<AppState>,
    Json(input): Json<CreateBoard>,
) -> Result<Json<Board>, AppError> {
    Ok(Json(svc::create_board(&state.pool, input).await?))
}

async fn get_board(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BoardWithColumns>, AppError> {
    Ok(Json(svc::get_board(&state.pool, id).await?))
}

async fn delete_board(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    svc::delete_board(&state.pool, id).await
}

async fn create_column(
    State(state): State<AppState>,
    Path(board_id): Path<Uuid>,
    Json(input): Json<CreateColumn>,
) -> Result<Json<Column>, AppError> {
    Ok(Json(svc::create_column(&state.pool, board_id, input).await?))
}

async fn create_card(
    State(state): State<AppState>,
    Path(column_id): Path<Uuid>,
    Json(input): Json<CreateCard>,
) -> Result<Json<Card>, AppError> {
    Ok(Json(svc::create_card(&state.pool, column_id, input).await?))
}

async fn update_card(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateCard>,
) -> Result<Json<Card>, AppError> {
    Ok(Json(svc::update_card(&state.pool, id, input).await?))
}

async fn delete_card(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    svc::delete_card(&state.pool, id).await
}
