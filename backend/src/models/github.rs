use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GithubRepo {
    pub id: Uuid,
    pub owner: String,
    pub name: String,
    pub last_synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GithubIssue {
    pub id: Uuid,
    pub repo_id: Uuid,
    pub issue_number: i32,
    pub title: String,
    pub state: String,
    pub labels: Option<Vec<String>>,
    pub body: Option<String>,
    pub card_id: Option<Uuid>,
    pub synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GithubPullRequest {
    pub id: Uuid,
    pub repo_id: Uuid,
    pub pr_number: i32,
    pub title: String,
    pub state: String,
    pub linked_issue_number: Option<i32>,
    pub card_id: Option<Uuid>,
    pub synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectRepo {
    pub owner: String,
    pub name: String,
}
