use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Board {
    pub id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Column {
    pub id: Uuid,
    pub board_id: Uuid,
    pub name: String,
    pub position: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Card {
    pub id: Uuid,
    pub column_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub position: i32,
    pub due_date: Option<DateTime<Utc>>,
    pub labels: Option<Vec<String>>,
    pub github_issue_id: Option<i32>,
    pub github_pr_id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBoard {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBoard {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateColumn {
    pub name: String,
    pub position: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCard {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCard {
    pub title: Option<String>,
    pub description: Option<String>,
    pub column_id: Option<Uuid>,
    pub position: Option<i32>,
    pub due_date: Option<DateTime<Utc>>,
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct BoardWithColumns {
    #[serde(flatten)]
    pub board: Board,
    pub columns: Vec<ColumnWithCards>,
}

#[derive(Debug, Serialize)]
pub struct ColumnWithCards {
    #[serde(flatten)]
    pub column: Column,
    pub cards: Vec<Card>,
}
