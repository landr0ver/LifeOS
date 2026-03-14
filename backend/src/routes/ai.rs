use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::services::ai;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/ai/chat", post(chat))
        .route("/api/ai/summarize", post(summarize))
        .route("/api/ai/suggest-tasks", post(suggest_tasks))
        .route("/api/ai/connect-notes", post(connect_notes))
}

#[derive(Deserialize)]
struct ChatRequest {
    messages: Vec<ai::ClaudeMessage>,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
}

async fn chat(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    let api_key = state
        .config
        .claude_api_key
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Claude API key not configured".to_string()))?;

    let response = ai::chat(
        api_key,
        req.messages,
        Some("You are LifeOS, a personal planning and organization assistant. Help the user plan tasks, organize ideas, and stay productive."),
    )
    .await?;

    Ok(Json(ChatResponse { response }))
}

#[derive(Deserialize)]
struct SummarizeRequest {
    content: String,
}

async fn summarize(
    State(state): State<AppState>,
    Json(req): Json<SummarizeRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    let api_key = state
        .config
        .claude_api_key
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Claude API key not configured".to_string()))?;

    let messages = vec![ai::ClaudeMessage {
        role: "user".to_string(),
        content: format!("Summarize the following content concisely:\n\n{}", req.content),
    }];

    let response = ai::chat(api_key, messages, None).await?;
    Ok(Json(ChatResponse { response }))
}

#[derive(Deserialize)]
struct SuggestTasksRequest {
    context: String,
}

async fn suggest_tasks(
    State(state): State<AppState>,
    Json(req): Json<SuggestTasksRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    let api_key = state
        .config
        .claude_api_key
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Claude API key not configured".to_string()))?;

    let messages = vec![ai::ClaudeMessage {
        role: "user".to_string(),
        content: format!(
            "Based on the following project context, suggest 3-5 actionable next tasks. \
             Return them as a JSON array of strings.\n\nContext:\n{}",
            req.context
        ),
    }];

    let response = ai::chat(api_key, messages, None).await?;
    Ok(Json(ChatResponse { response }))
}

#[derive(Deserialize)]
struct ConnectNotesRequest {
    notes: Vec<NoteSnippet>,
}

#[derive(Deserialize)]
struct NoteSnippet {
    #[allow(dead_code)]
    id: String,
    title: String,
    content: String,
}

async fn connect_notes(
    State(state): State<AppState>,
    Json(req): Json<ConnectNotesRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    let api_key = state
        .config
        .claude_api_key
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Claude API key not configured".to_string()))?;

    let notes_text: String = req
        .notes
        .iter()
        .map(|n| format!("- {}: {}", n.title, &n.content[..n.content.len().min(200)]))
        .collect::<Vec<_>>()
        .join("\n");

    let messages = vec![ai::ClaudeMessage {
        role: "user".to_string(),
        content: format!(
            "Analyze these notes and suggest connections between them. \
             Return a JSON array of objects with 'source_id', 'target_id', and 'reason' fields.\n\nNotes:\n{}",
            notes_text
        ),
    }];

    let response = ai::chat(api_key, messages, None).await?;
    Ok(Json(ChatResponse { response }))
}
