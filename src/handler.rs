use std::any::{Any, TypeId};

use crate::{
    dto::{CreateNoteDto, EditNoteDto},
    errors::sqlx_error_handler,
    models::Note,
    repo::NoteRepo,
    responses::{AppError, AppResponse},
};

#[derive(Clone)]
pub struct NoteHandler {
    repo: NoteRepo,
}

impl NoteHandler {
    pub fn new(repo: NoteRepo) -> Self {
        Self { repo }
    }

    pub async fn get_notes_by_username(
        &self,
        username: &str,
    ) -> Result<AppResponse<Vec<Note>>, AppError> {
        let result = self.repo.get_notes(username).await;
        match result {
            Ok(notes) => {
                if notes.is_empty() {
                    Err(AppError::NotFound {
                        message: "Notes not found".to_string(),
                    })
                } else {
                    Ok(AppResponse::OK(notes))
                }
            }
            Err(e) => {
                if e.type_id() == TypeId::of::<sqlx::Error>() {
                    Err(sqlx_error_handler(e).await)
                } else {
                    Err(AppError::InternalServerError {
                        message: format!("Server error: {}", e),
                    })
                }
            }
        }
    }

    pub async fn get_note_by_id(&self, id: i32) -> Result<AppResponse<Note>, AppError> {
        let result = self.repo.get_note(id).await;
        match result {
            Ok(note) => Ok(AppResponse::OK(note)),
            Err(e) => {
                if e.type_id() == TypeId::of::<sqlx::Error>() {
                    Err(sqlx_error_handler(e).await)
                } else {
                    Err(AppError::InternalServerError {
                        message: format!("Server error: {}", e),
                    })
                }
            }
        }
    }

    pub async fn create_note(
        &self,
        payload: &CreateNoteDto,
    ) -> Result<AppResponse<Note>, AppError> {
        let result = self.repo.create_note(payload).await;
        match result {
            Ok(note) => Ok(AppResponse::Created(note)),
            Err(e) => {
                if e.type_id() == TypeId::of::<sqlx::Error>() {
                    Err(sqlx_error_handler(e).await)
                } else {
                    Err(AppError::InternalServerError {
                        message: format!("Server error: {}", e),
                    })
                }
            }
        }
    }

    pub async fn edit_note(&self, payload: &EditNoteDto) -> Result<AppResponse<()>, AppError> {
        let result = self.repo.edit_note(payload).await;
        match result {
            Ok(()) => Ok(AppResponse::OK(())),
            Err(e) => {
                if e.type_id() == TypeId::of::<sqlx::Error>() {
                    Err(sqlx_error_handler(e).await)
                } else {
                    Err(AppError::InternalServerError {
                        message: format!("Server error: {}", e),
                    })
                }
            }
        }
    }

    pub async fn delete_note(&self, id: i32) -> Result<AppResponse<()>, AppError> {
        let result = self.repo.delete_note(id).await;
        match result {
            Ok(()) => Ok(AppResponse::OK(())),
            Err(e) => {
                if e.type_id() == TypeId::of::<sqlx::Error>() {
                    Err(sqlx_error_handler(e).await)
                } else {
                    Err(AppError::InternalServerError {
                        message: format!("Server error: {}", e),
                    })
                }
            }
        }
    }
}
