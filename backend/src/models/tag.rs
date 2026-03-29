use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTag {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            created_at: chrono::Utc::now(),
        }
    }
}

// Junction table for note-tag relationships
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NoteTag {
    pub note_id: Uuid,
    pub tag_id: Uuid,
}
