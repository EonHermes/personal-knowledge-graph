use crate::models::{CreateNote, Note, UpdateNote};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub struct NoteRepository {
    pool: Arc<SqlitePool>,
}

impl NoteRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, note: CreateNote) -> Result<Note, sqlx::Error> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4().to_string();
        let note_type = note.note_type.unwrap_or_else(|| "note".to_string());

        let row = sqlx::query_as::<_, Note>(
            r#"
            INSERT INTO notes (id, title, content, note_type, url, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&note.title)
        .bind(&note.content)
        .bind(&note_type)
        .bind(&note.url)
        .bind(now)
        .bind(now)
        .fetch_one(&*self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Note>, sqlx::Error> {
        let result = sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&*self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_all(&self) -> Result<Vec<Note>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Note>(
            "SELECT * FROM notes ORDER BY created_at DESC",
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn update(&self, id: Uuid, update: UpdateNote) -> Result<Note, sqlx::Error> {
        let now = chrono::Utc::now();

        // Get existing note first
        let existing = self.get_by_id(id).await?
            .ok_or(sqlx::Error::RowNotFound)?;

        // Use provided values or keep existing
        let title = update.title.unwrap_or(existing.title);
        let content = update.content.unwrap_or(existing.content);
        let url = update.url.or(existing.url);

        sqlx::query(
            "UPDATE notes SET title = ?, content = ?, url = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&title)
        .bind(&content)
        .bind(&url)
        .bind(now)
        .bind(id.to_string())
        .execute(&*self.pool)
        .await?;

        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(id.to_string())
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Note>, sqlx::Error> {
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query_as::<_, Note>(
            r#"
            SELECT * FROM notes 
            WHERE title LIKE ? OR content LIKE ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_by_type(&self, note_type: &str) -> Result<Vec<Note>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE note_type = ? ORDER BY created_at DESC",
        )
        .bind(note_type)
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows)
    }
}
