use crate::models::{Connection, CreateConnection};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub struct ConnectionRepository {
    pool: Arc<SqlitePool>,
}

impl ConnectionRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, conn: CreateConnection) -> Result<Connection, sqlx::Error> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4().to_string();
        let connection_type = conn.connection_type.unwrap_or_else(|| "manual".to_string());
        let strength = conn.strength.unwrap_or(1.0);

        // Check if connection already exists (in either direction)
        let existing = sqlx::query_as::<_, Connection>(
            r#"
            SELECT * FROM connections 
            WHERE (source_note_id = ? AND target_note_id = ?)
               OR (source_note_id = ? AND target_note_id = ?)
            "#,
        )
        .bind(conn.source_note_id.to_string())
        .bind(conn.target_note_id.to_string())
        .bind(conn.target_note_id.to_string())
        .bind(conn.source_note_id.to_string())
        .fetch_optional(&*self.pool)
        .await?;

        if existing.is_some() {
            return Err(sqlx::Error::Protocol("Connection already exists".to_string()));
        }

        let row = sqlx::query_as::<_, Connection>(
            r#"
            INSERT INTO connections (id, source_note_id, target_note_id, connection_type, strength, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(conn.source_note_id.to_string())
        .bind(conn.target_note_id.to_string())
        .bind(&connection_type)
        .bind(strength)
        .bind(now)
        .fetch_one(&*self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Connection>, sqlx::Error> {
        let result = sqlx::query_as::<_, Connection>(
            "SELECT * FROM connections WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&*self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_all(&self) -> Result<Vec<Connection>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Connection>(
            "SELECT * FROM connections ORDER BY created_at DESC",
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_connections_for_note(&self, note_id: Uuid) -> Result<Vec<Connection>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Connection>(
            r#"
            SELECT * FROM connections 
            WHERE source_note_id = ? OR target_note_id = ?
            ORDER BY strength DESC
            "#,
        )
        .bind(note_id.to_string())
        .bind(note_id.to_string())
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_connections_between(&self, source_id: Uuid, target_id: Uuid) -> Result<Option<Connection>, sqlx::Error> {
        let result = sqlx::query_as::<_, Connection>(
            r#"
            SELECT * FROM connections 
            WHERE (source_note_id = ? AND target_note_id = ?)
               OR (source_note_id = ? AND target_note_id = ?)
            "#,
        )
        .bind(source_id.to_string())
        .bind(target_id.to_string())
        .bind(target_id.to_string())
        .bind(source_id.to_string())
        .fetch_optional(&*self.pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM connections WHERE id = ?")
            .bind(id.to_string())
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_connections_for_note(&self, note_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM connections WHERE source_note_id = ? OR target_note_id = ?")
            .bind(note_id.to_string())
            .bind(note_id.to_string())
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_strength(&self, id: Uuid, strength: f32) -> Result<Connection, sqlx::Error> {
        sqlx::query("UPDATE connections SET strength = ? WHERE id = ?")
            .bind(strength)
            .bind(id.to_string())
            .execute(&*self.pool)
            .await?;

        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }
}
