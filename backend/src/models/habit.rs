use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Habit {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub frequency: String,
    pub target_days: Option<Vec<i32>>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct HabitEntry {
    pub id: Uuid,
    pub habit_id: Uuid,
    pub date: NaiveDate,
    pub completed: Option<bool>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateHabit {
    pub name: String,
    pub description: Option<String>,
    pub frequency: String,
    pub target_days: Option<Vec<i32>>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateHabitEntry {
    pub date: NaiveDate,
    pub completed: bool,
    pub note: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HabitStats {
    pub habit_id: Uuid,
    pub current_streak: i32,
    pub longest_streak: i32,
    pub completion_rate: f64,
    pub total_entries: i64,
}
