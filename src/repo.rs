use sqlx::{SqliteConnection, query};

use crate::{dto::CreateNoteDto, models::Note};

// Get notes by username
async fn get_notes(username: &str, pool: &mut SqliteConnection) -> Result<Vec<Note>, sqlx::Error> {
    let notes: Vec<Note> = sqlx::query_as(
        "SELECT id, username, title, body, last_edited, created_at FROM notes WHERE username = ?",
    )
    .bind(username)
    .fetch_all(pool)
    .await?;

    Ok(notes)
}
// Get note by id
async fn get_note(id: i32, pool: &mut SqliteConnection) -> Result<Note, sqlx::Error> {
    let notes: Note = sqlx::query_as(
        "SELECT id, username, title, body, last_edited, created_at FROM notes WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(notes)
}
// Create note
async fn create_note(
    input: &CreateNoteDto,
    pool: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO notes (username, title, body, last_edited) VALUES (?, ?, ?, ?,)")
        .bind(&input.username)
        .bind(&input.title)
        .bind(&input.body)
        .execute(pool)
        .await?;

    Ok(())
}
// Edit note
async fn edit_note(
    id: i32,
    title: &str,
    body: &str,
    pool: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE notes
        SET body = COALESCE(?, body),
            title = COALESCE(?, title)
            last_edited = CURRENT_TIMESTAMP
        WHERE id = ?",
    )
    .bind(body)
    .bind(title)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
// Delete note
async fn delete_note(id: i32, pool: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
