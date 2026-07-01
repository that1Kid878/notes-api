use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow, sqlx::Type)]
pub struct Note {
    pub id: i32,
    pub username: String,
    pub title: String,
    pub body: String,
    pub last_edited: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
