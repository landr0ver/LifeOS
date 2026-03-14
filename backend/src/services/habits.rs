use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::habit::*;

pub async fn list_habits(pool: &PgPool) -> Result<Vec<Habit>, AppError> {
    let habits = sqlx::query_as::<_, Habit>(
        "SELECT * FROM habits WHERE archived_at IS NULL ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(habits)
}

pub async fn create_habit(pool: &PgPool, input: CreateHabit) -> Result<Habit, AppError> {
    let habit = sqlx::query_as::<_, Habit>(
        "INSERT INTO habits (name, description, frequency, target_days, color, icon) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.frequency)
    .bind(&input.target_days)
    .bind(&input.color)
    .bind(&input.icon)
    .fetch_one(pool)
    .await?;
    Ok(habit)
}

pub async fn update_habit(pool: &PgPool, id: Uuid, input: CreateHabit) -> Result<Habit, AppError> {
    let habit = sqlx::query_as::<_, Habit>(
        "UPDATE habits SET name = $2, description = $3, frequency = $4, \
         target_days = $5, color = $6, icon = $7 WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.frequency)
    .bind(&input.target_days)
    .bind(&input.color)
    .bind(&input.icon)
    .fetch_one(pool)
    .await?;
    Ok(habit)
}

pub async fn delete_habit(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("UPDATE habits SET archived_at = now() WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn log_entry(
    pool: &PgPool,
    habit_id: Uuid,
    input: CreateHabitEntry,
) -> Result<HabitEntry, AppError> {
    let entry = sqlx::query_as::<_, HabitEntry>(
        "INSERT INTO habit_entries (habit_id, date, completed, note) \
         VALUES ($1, $2, $3, $4) \
         ON CONFLICT (habit_id, date) DO UPDATE SET completed = $3, note = $4 \
         RETURNING *",
    )
    .bind(habit_id)
    .bind(input.date)
    .bind(input.completed)
    .bind(&input.note)
    .fetch_one(pool)
    .await?;
    Ok(entry)
}

pub async fn get_entries(
    pool: &PgPool,
    habit_id: Uuid,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<HabitEntry>, AppError> {
    let entries = sqlx::query_as::<_, HabitEntry>(
        "SELECT * FROM habit_entries WHERE habit_id = $1 AND date >= $2 AND date <= $3 ORDER BY date",
    )
    .bind(habit_id)
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;
    Ok(entries)
}
