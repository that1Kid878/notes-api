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
