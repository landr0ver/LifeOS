use axum::extract::{Path, Query, State};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::note::*;
use crate::services::notes as svc;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/notes", get(list_notes).post(create_note))
        .route("/api/notes/graph", get(get_graph))
        .route("/api/notes/search", get(search_notes))
        .route("/api/notes/{id}", get(get_note).put(update_note).delete(delete_note))
        .route("/api/notes/{id}/links", post(create_link))
        .route("/api/notes/links/{id}", delete(delete_link))
}

async fn list_notes(State(state): State<AppState>) -> Result<Json<Vec<Note>>, AppError> {
    Ok(Json(svc::list_notes(&state.pool).await?))
}

async fn create_note(
    State(state): State<AppState>,
    Json(input): Json<CreateNote>,
) -> Result<Json<Note>, AppError> {
    Ok(Json(svc::create_note(&state.pool, input).await?))
}

async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Note>, AppError> {
    Ok(Json(svc::get_note(&state.pool, id).await?))
}

async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateNote>,
) -> Result<Json<Note>, AppError> {
    Ok(Json(svc::update_note(&state.pool, id, input).await?))
}

async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    svc::delete_note(&state.pool, id).await
}

async fn get_graph(State(state): State<AppState>) -> Result<Json<GraphData>, AppError> {
    Ok(Json(svc::get_graph(&state.pool).await?))
}

async fn create_link(
    State(state): State<AppState>,
    Path(source_id): Path<Uuid>,
    Json(input): Json<CreateNoteLink>,
) -> Result<Json<NoteLink>, AppError> {
    Ok(Json(svc::create_link(&state.pool, source_id, input).await?))
}

async fn delete_link(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    svc::delete_link(&state.pool, id).await
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

async fn search_notes(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<Note>>, AppError> {
    Ok(Json(svc::search_notes(&state.pool, &query.q).await?))
}
