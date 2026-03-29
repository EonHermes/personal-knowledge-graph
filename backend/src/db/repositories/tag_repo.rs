use crate::models::{CreateTag, NoteTag, Tag};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub struct TagRepository {
    pool: Arc<SqlitePool>,
}

impl TagRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, tag: CreateTag) -> Result<Tag, sqlx::Error> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4().to_string();

        let row = sqlx::query_as::<_, Tag>(
            r#"
            INSERT INTO tags (id, name, created_at)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&tag.name)
        .bind(now)
        .fetch_one(&*self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Tag>, sqlx::Error> {
        let result = sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&*self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Option<Tag>, sqlx::Error> {
        let result = sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags WHERE name = ?",
        )
        .bind(name)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_all(&self) -> Result<Vec<Tag>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags ORDER BY name ASC",
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tags WHERE id = ?")
            .bind(id.to_string())
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    // Note-Tag relationships
    pub async fn add_tag_to_note(&self, note_id: Uuid, tag_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?, ?)",
        )
        .bind(note_id.to_string())
        .bind(tag_id.to_string())
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    pub async fn remove_tag_from_note(&self, note_id: Uuid, tag_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM note_tags WHERE note_id = ? AND tag_id = ?",
        )
        .bind(note_id.to_string())
        .bind(tag_id.to_string())
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_tags_for_note(&self, note_id: Uuid) -> Result<Vec<Tag>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Tag>(
            r#"
            SELECT t.* FROM tags t
            INNER JOIN note_tags nt ON t.id = nt.tag_id
            WHERE nt.note_id = ?
            "#,
        )
        .bind(note_id.to_string())
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_notes_for_tag(&self, tag_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        let rows = sqlx::query_scalar::<_, String>(
            "SELECT note_id FROM note_tags WHERE tag_id = ?",
        )
        .bind(tag_id.to_string())
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows.into_iter().map(|s| Uuid::parse_str(&s).unwrap()).collect())
    }
}
