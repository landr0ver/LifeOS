use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub all_day: Option<bool>,
    pub recurrence: Option<String>,
    pub color: Option<String>,
    pub card_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEvent {
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub all_day: Option<bool>,
    pub recurrence: Option<String>,
    pub color: Option<String>,
    pub card_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEvent {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub all_day: Option<bool>,
    pub recurrence: Option<String>,
    pub color: Option<String>,
    pub card_id: Option<Uuid>,
}
