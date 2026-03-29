use crate::db::repositories::{ConnectionRepository, NoteRepository};
use crate::models::{CreateConnection, Note};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct SuggestionService {
    note_repo: NoteRepository,
    connection_repo: ConnectionRepository,
}

impl SuggestionService {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            note_repo: NoteRepository::new(pool.clone()),
            connection_repo: ConnectionRepository::new(pool),
        }
    }

    /// Calculate simple text similarity between two notes using word overlap
    fn calculate_similarity(note1: &Note, note2: &Note) -> f32 {
        let words1 = Self::extract_words(&note1.content);
        let words2 = Self::extract_words(&note2.content);

        if words1.is_empty() || words2.is_empty() {
            return 0.0;
        }

        let intersection: usize = words1
            .iter()
            .filter(|w| words2.contains(w))
            .count();

        let union = words1.len() + words2.len() - intersection;
        
        if union == 0 {
            0.0
        } else {
            (intersection as f32) / (union as f32)
        }
    }

    fn extract_words(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .filter(|w| w.len() > 3) // Skip short words
            .map(|w| w.chars().filter(|c| c.is_alphabetic()).collect::<String>())
            .filter(|w: &String| !w.is_empty())
            .collect()
    }

    /// Get suggested connections for a specific note
    pub async fn suggest_connections_for_note(&self, note_id: Uuid) -> Result<Vec<(Uuid, f32)>, Box<dyn std::error::Error + Send + Sync>> {
        let target_note = self.note_repo.get_by_id(note_id).await?
            .ok_or("Note not found")?;

        let all_notes = self.note_repo.get_all().await?;
        
        // Get existing connections to exclude
        let existing_connections = self.connection_repo.get_connections_for_note(note_id).await?;
        let mut excluded_ids: Vec<Uuid> = existing_connections
            .iter()
            .map(|c| {
                if c.source_note_id == note_id {
                    c.target_note_id
                } else {
                    c.source_note_id
                }
            })
            .collect();
        excluded_ids.push(note_id); // Don't connect to self

        let mut suggestions: Vec<(Uuid, f32)> = Vec::new();

        for other_note in all_notes {
            if excluded_ids.contains(&other_note.id) {
                continue;
            }

            let similarity = Self::calculate_similarity(&target_note, &other_note);
            
            // Only suggest if similarity is above threshold
            if similarity > 0.15 {
                suggestions.push((other_note.id, similarity));
            }
        }

        // Sort by similarity descending and take top 10
        suggestions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(suggestions.into_iter().take(10).collect())
    }

    /// Get all suggested connections (for notes without many connections)
    pub async fn get_all_suggestions(&self) -> Result<HashMap<Uuid, Vec<(Uuid, f32)>>, Box<dyn std::error::Error + Send + Sync>> {
        let notes = self.note_repo.get_all().await?;
        let mut suggestions_map: HashMap<Uuid, Vec<(Uuid, f32)>> = HashMap::new();

        for note in notes {
            // Only suggest for notes with fewer than 5 connections
            let existing_count = self.connection_repo.get_connections_for_note(note.id).await?.len();
            if existing_count < 5 {
                let suggestions = self.suggest_connections_for_note(note.id).await?;
                if !suggestions.is_empty() {
                    suggestions_map.insert(note.id, suggestions);
                }
            }
        }

        Ok(suggestions_map)
    }

    /// Create a suggested connection
    pub async fn create_suggested_connection(&self, source_id: Uuid, target_id: Uuid, strength: f32) -> Result<CreateConnection, Box<dyn std::error::Error + Send + Sync>> {
        Ok(CreateConnection {
            source_note_id: source_id,
            target_note_id: target_id,
            connection_type: Some("suggested".to_string()),
            strength: Some(strength),
        })
    }
}
