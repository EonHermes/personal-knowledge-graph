use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Connection {
    pub id: Uuid,
    pub source_note_id: Uuid,
    pub target_note_id: Uuid,
    pub connection_type: String, // "manual", "suggested", "auto"
    pub strength: f32, // 0.0 to 1.0
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateConnection {
    pub source_note_id: Uuid,
    pub target_note_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub connection_type: Option<String>,
    pub strength: Option<f32>,
}

impl Connection {
    pub fn new(source_note_id: Uuid, target_note_id: Uuid, connection_type: Option<String>, strength: Option<f32>) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_note_id,
            target_note_id,
            connection_type: connection_type.unwrap_or_else(|| "manual".to_string()),
            strength: strength.unwrap_or(1.0),
            created_at: chrono::Utc::now(),
        }
    }
}
