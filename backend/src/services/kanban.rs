use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::kanban::*;

pub async fn list_boards(pool: &PgPool) -> Result<Vec<Board>, AppError> {
    let boards = sqlx::query_as::<_, Board>("SELECT * FROM boards ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;
    Ok(boards)
}

pub async fn create_board(pool: &PgPool, input: CreateBoard) -> Result<Board, AppError> {
    let board = sqlx::query_as::<_, Board>(
        "INSERT INTO boards (name) VALUES ($1) RETURNING *",
    )
    .bind(&input.name)
    .fetch_one(pool)
    .await?;
    Ok(board)
}

pub async fn get_board(pool: &PgPool, id: Uuid) -> Result<BoardWithColumns, AppError> {
    let board = sqlx::query_as::<_, Board>("SELECT * FROM boards WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Board not found".to_string()))?;

    let columns = sqlx::query_as::<_, Column>(
        "SELECT * FROM columns WHERE board_id = $1 ORDER BY position",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let mut columns_with_cards = Vec::new();
    for col in columns {
        let cards = sqlx::query_as::<_, Card>(
            "SELECT * FROM cards WHERE column_id = $1 ORDER BY position",
        )
        .bind(col.id)
        .fetch_all(pool)
        .await?;
        columns_with_cards.push(ColumnWithCards {
            column: col,
            cards,
        });
    }

    Ok(BoardWithColumns {
        board,
        columns: columns_with_cards,
    })
}

pub async fn delete_board(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM boards WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn create_column(
    pool: &PgPool,
    board_id: Uuid,
    input: CreateColumn,
) -> Result<Column, AppError> {
    let position = match input.position {
        Some(p) => p,
        None => {
            sqlx::query_scalar::<_, Option<i32>>(
                "SELECT MAX(position) FROM columns WHERE board_id = $1",
            )
            .bind(board_id)
            .fetch_one(pool)
            .await?
            .unwrap_or(0)
                + 1
        }
    };

    let column = sqlx::query_as::<_, Column>(
        "INSERT INTO columns (board_id, name, position) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(board_id)
    .bind(&input.name)
    .bind(position)
    .fetch_one(pool)
    .await?;
    Ok(column)
}

pub async fn create_card(
    pool: &PgPool,
    column_id: Uuid,
    input: CreateCard,
) -> Result<Card, AppError> {
    let position = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT MAX(position) FROM cards WHERE column_id = $1",
    )
    .bind(column_id)
    .fetch_one(pool)
    .await?
    .unwrap_or(0)
        + 1;

    let card = sqlx::query_as::<_, Card>(
        "INSERT INTO cards (column_id, title, description, position, due_date, labels) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(column_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(position)
    .bind(input.due_date)
    .bind(&input.labels)
    .fetch_one(pool)
    .await?;
    Ok(card)
}

pub async fn update_card(pool: &PgPool, id: Uuid, input: UpdateCard) -> Result<Card, AppError> {
    let card = sqlx::query_as::<_, Card>(
        "UPDATE cards SET \
         title = COALESCE($2, title), \
         description = COALESCE($3, description), \
         column_id = COALESCE($4, column_id), \
         position = COALESCE($5, position), \
         due_date = COALESCE($6, due_date), \
         labels = COALESCE($7, labels), \
         updated_at = now() \
         WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.column_id)
    .bind(input.position)
    .bind(input.due_date)
    .bind(&input.labels)
    .fetch_one(pool)
    .await?;
    Ok(card)
}

pub async fn delete_card(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM cards WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
