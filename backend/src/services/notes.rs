use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::note::*;

pub async fn list_notes(pool: &PgPool) -> Result<Vec<Note>, AppError> {
    let notes = sqlx::query_as::<_, Note>("SELECT * FROM notes ORDER BY updated_at DESC")
        .fetch_all(pool)
        .await?;
    Ok(notes)
}

pub async fn create_note(pool: &PgPool, input: CreateNote) -> Result<Note, AppError> {
    let note = sqlx::query_as::<_, Note>(
        "INSERT INTO notes (title, content, tags) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&input.title)
    .bind(&input.content)
    .bind(&input.tags)
    .fetch_one(pool)
    .await?;
    Ok(note)
}

pub async fn get_note(pool: &PgPool, id: Uuid) -> Result<Note, AppError> {
    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Note not found".to_string()))?;
    Ok(note)
}

pub async fn update_note(pool: &PgPool, id: Uuid, input: UpdateNote) -> Result<Note, AppError> {
    let note = sqlx::query_as::<_, Note>(
        "UPDATE notes SET \
         title = COALESCE($2, title), \
         content = COALESCE($3, content), \
         tags = COALESCE($4, tags), \
         updated_at = now() \
         WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(&input.title)
    .bind(&input.content)
    .bind(&input.tags)
    .fetch_one(pool)
    .await?;
    Ok(note)
}

pub async fn delete_note(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_graph(pool: &PgPool) -> Result<GraphData, AppError> {
    let nodes = sqlx::query_as::<_, (Uuid, String, Option<Vec<String>>)>(
        "SELECT id, title, tags FROM notes",
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(id, title, tags)| GraphNode { id, title, tags })
    .collect();

    let edges =
        sqlx::query_as::<_, (Uuid, Uuid, Uuid, Option<String>)>(
            "SELECT id, source_id, target_id, label FROM note_links",
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|(id, source, target, label)| GraphEdge {
            id,
            source,
            target,
            label,
        })
        .collect();

    Ok(GraphData { nodes, edges })
}

pub async fn create_link(
    pool: &PgPool,
    source_id: Uuid,
    input: CreateNoteLink,
) -> Result<NoteLink, AppError> {
    let link = sqlx::query_as::<_, NoteLink>(
        "INSERT INTO note_links (source_id, target_id, label) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(source_id)
    .bind(input.target_id)
    .bind(&input.label)
    .fetch_one(pool)
    .await?;
    Ok(link)
}

pub async fn delete_link(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM note_links WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn search_notes(pool: &PgPool, query: &str) -> Result<Vec<Note>, AppError> {
    let notes = sqlx::query_as::<_, Note>(
        "SELECT * FROM notes \
         WHERE to_tsvector('english', title || ' ' || content) @@ plainto_tsquery('english', $1) \
         ORDER BY ts_rank(to_tsvector('english', title || ' ' || content), plainto_tsquery('english', $1)) DESC",
    )
    .bind(query)
    .fetch_all(pool)
    .await?;
    Ok(notes)
}
