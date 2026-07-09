use sqlx::{SqlitePool, query, query_as};

use crate::{
    dto::{CreateNoteDto, EditNoteDto},
    models::Note,
};

#[derive(Clone)]
pub struct NoteRepo {
    pool: SqlitePool,
}

impl NoteRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // Get notes by username
    pub async fn get_notes(&self, username: &str) -> Result<Vec<Note>, sqlx::Error> {
        let notes: Vec<Note> = query_as(
            "SELECT id, username, title, body, last_edited, created_at FROM notes WHERE username = ?",
        )
        .bind(username)
        .fetch_all(&self.pool)
        .await?;

        Ok(notes)
    }
    // Get note by id
    pub async fn get_note(&self, id: i32) -> Result<Note, sqlx::Error> {
        let notes: Note = query_as(
            "SELECT id, username, title, body, last_edited, created_at FROM notes WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(notes)
    }
    // Create note
    pub async fn create_note(&self, input: &CreateNoteDto) -> Result<Note, sqlx::Error> {
        let note: Note =
            query_as("INSERT INTO notes (username, title, body, last_edited) VALUES (?, ?, ?) RETURNING id, username, title, body, last_edited, created_at")
                .bind(&input.username)
                .bind(&input.title)
                .bind(&input.body)
                .fetch_one(&self.pool)
                .await?;

        Ok(note)
    }
    // Edit note
    pub async fn edit_note(&self, input: &EditNoteDto) -> Result<(), sqlx::Error> {
        query(
            "UPDATE notes
            SET body = COALESCE(?, body),
                title = COALESCE(?, title)
                last_edited = CURRENT_TIMESTAMP
            WHERE id = ?",
        )
        .bind(&input.body)
        .bind(&input.title)
        .bind(&input.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    // Delete note
    pub async fn delete_note(&self, id: i32) -> Result<(), sqlx::Error> {
        query("DELETE FROM notes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
