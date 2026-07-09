use axum::{
    Json, Router,
    extract::State,
    routing::{delete, get, post, put},
};
use sqlx::sqlite::SqlitePoolOptions;

use crate::{
    dto::{CreateNoteDto, DeleteNoteDto, EditNoteDto, GetIdNotesDto, GetUsernameNotesDto},
    handler::NoteHandler,
    models::Note,
    repo::NoteRepo,
    responses::{AppError, AppResponse},
};

mod dto;
mod errors;
mod handler;
mod models;
mod repo;
mod responses;
#[derive(Clone)]
struct AppState {
    handler: NoteHandler,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://local.db")
        .await?;

    let repo = NoteRepo::new(pool);
    let handler = NoteHandler::new(repo.clone());
    let state = AppState {
        handler: handler.clone(),
    };
    let app = Router::new()
        .route("/notes/user/{username}", get(get_notes_by_username_route))
        .route("/notes/id/{id}", get(get_notes_by_id_route))
        .route("/notes/new", post(create_note_route))
        .route("/notes/edit", put(edit_note_route))
        .route("/notes/delete/{id}", delete(delet_note_route))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_notes_by_username_route(
    State(state): State<AppState>,
    Json(payload): Json<GetUsernameNotesDto>,
) -> Result<AppResponse<Vec<Note>>, AppError> {
    state.handler.get_notes_by_username(&payload.username).await
}

async fn get_notes_by_id_route(
    State(state): State<AppState>,
    Json(payload): Json<GetIdNotesDto>,
) -> Result<AppResponse<Note>, AppError> {
    state.handler.get_note_by_id(payload.id).await
}

async fn create_note_route(
    State(state): State<AppState>,
    Json(payload): Json<CreateNoteDto>,
) -> Result<AppResponse<()>, AppError> {
    state.handler.create_note(&payload).await
}

async fn edit_note_route(
    State(state): State<AppState>,
    Json(payload): Json<EditNoteDto>,
) -> Result<AppResponse<()>, AppError> {
    state.handler.edit_note(&payload).await
}

async fn delet_note_route(
    State(state): State<AppState>,
    Json(payload): Json<DeleteNoteDto>,
) -> Result<AppResponse<()>, AppError> {
    state.handler.delete_note(payload.id).await
}
