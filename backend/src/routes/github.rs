use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::error::AppError;
use crate::models::github::*;
use crate::services::github as svc;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/github/repos", get(list_repos).post(connect_repo))
        .route("/api/github/issues", get(list_issues))
        .route("/api/github/prs", get(list_prs))
        .route("/api/github/feature-sync", post(feature_sync))
}

async fn list_repos(State(state): State<AppState>) -> Result<Json<Vec<GithubRepo>>, AppError> {
    Ok(Json(svc::list_repos(&state.pool).await?))
}

async fn connect_repo(
    State(state): State<AppState>,
    Json(input): Json<ConnectRepo>,
) -> Result<Json<GithubRepo>, AppError> {
    let token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| AppError::BadRequest("GITHUB_TOKEN not configured".to_string()))?;
    Ok(Json(svc::connect_repo(&state.pool, input, &token).await?))
}

async fn list_issues(State(state): State<AppState>) -> Result<Json<Vec<GithubIssue>>, AppError> {
    Ok(Json(svc::list_issues(&state.pool).await?))
}

async fn list_prs(
    State(state): State<AppState>,
) -> Result<Json<Vec<GithubPullRequest>>, AppError> {
    Ok(Json(svc::list_prs(&state.pool).await?))
}

async fn feature_sync(State(state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    let content = std::fs::read_to_string("../feature.md")
        .map_err(|e| AppError::Internal(format!("Failed to read feature.md: {}", e)))?;

    let features = svc::parse_feature_md(&content);

    let repos = svc::list_repos(&state.pool).await?;
    let repo = repos
        .first()
        .ok_or_else(|| AppError::BadRequest("No GitHub repo connected".to_string()))?;

    let synced = svc::get_synced_features(&state.pool, repo.id).await?;

    let mut created = Vec::new();
    let token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| AppError::BadRequest("GITHUB_TOKEN not configured".to_string()))?;

    let octocrab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(|e| AppError::Internal(format!("Failed to create GitHub client: {}", e)))?;

    for feature in &features {
        if synced.contains(&feature.title) {
            continue;
        }

        let issue = octocrab
            .issues(&repo.owner, &repo.name)
            .create(&feature.title)
            .body(&feature.description)
            .labels(vec![feature.tag.clone()])
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to create GitHub issue: {}", e)))?;

        svc::log_feature_sync(&state.pool, repo.id, &feature.title, issue.number as i32).await?;
        created.push(feature.title.clone());
    }

    Ok(Json(serde_json::json!({
        "created_issues": created,
        "total_features": features.len(),
        "already_synced": synced.len(),
    })))
}
