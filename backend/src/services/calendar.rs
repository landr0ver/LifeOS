use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::calendar::*;

pub async fn list_events(
    pool: &PgPool,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<Vec<Event>, AppError> {
    let events = sqlx::query_as::<_, Event>(
        "SELECT * FROM events WHERE start_time >= $1 AND start_time <= $2 ORDER BY start_time",
    )
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;
    Ok(events)
}

pub async fn create_event(pool: &PgPool, input: CreateEvent) -> Result<Event, AppError> {
    let event = sqlx::query_as::<_, Event>(
        "INSERT INTO events (title, description, start_time, end_time, all_day, recurrence, color, card_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
    )
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.start_time)
    .bind(input.end_time)
    .bind(input.all_day)
    .bind(&input.recurrence)
    .bind(&input.color)
    .bind(input.card_id)
    .fetch_one(pool)
    .await?;
    Ok(event)
}

pub async fn update_event(
    pool: &PgPool,
    id: Uuid,
    input: UpdateEvent,
) -> Result<Event, AppError> {
    let event = sqlx::query_as::<_, Event>(
        "UPDATE events SET \
         title = COALESCE($2, title), \
         description = COALESCE($3, description), \
         start_time = COALESCE($4, start_time), \
         end_time = COALESCE($5, end_time), \
         all_day = COALESCE($6, all_day), \
         recurrence = COALESCE($7, recurrence), \
         color = COALESCE($8, color), \
         card_id = COALESCE($9, card_id) \
         WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.start_time)
    .bind(input.end_time)
    .bind(input.all_day)
    .bind(&input.recurrence)
    .bind(&input.color)
    .bind(input.card_id)
    .fetch_one(pool)
    .await?;
    Ok(event)
}

pub async fn delete_event(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM events WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
