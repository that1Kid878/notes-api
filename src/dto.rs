use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateNoteDto {
    pub username: String,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EditNoteDto {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetUsernameNotesDto {
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetIdNotesDto {
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeleteNoteDto {
    pub id: i32,
}
