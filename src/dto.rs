use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateNoteDto {
    pub username: String,
    pub title: String,
    pub body: String,
}
