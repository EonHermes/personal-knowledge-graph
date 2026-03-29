use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub note_type: String, // "note" or "bookmark"
    pub url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateNote {
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    
    #[validate(length(min = 1))]
    pub content: String,
    
    #[validate(length(max = 10))]
    pub note_type: Option<String>,
    
    #[validate(url)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
    pub url: Option<String>,
}

impl Note {
    pub fn new(title: String, content: String, note_type: Option<String>, url: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            note_type: note_type.unwrap_or_else(|| "note".to_string()),
            url,
            created_at: now,
            updated_at: now,
        }
    }
}
