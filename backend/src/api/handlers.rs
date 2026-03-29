use crate::db::repositories::{ConnectionRepository, NoteRepository, TagRepository};
use crate::models::{Connection, CreateConnection, CreateNote, CreateTag, Note, Tag, UpdateNote};
use crate::services::SuggestionService;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

// Application state
#[derive(Clone)]
pub struct AppState {
    pub note_repo: Arc<NoteRepository>,
    pub tag_repo: Arc<TagRepository>,
    pub connection_repo: Arc<ConnectionRepository>,
    pub suggestion_service: Arc<SuggestionService>,
}

// Response types
#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

// Note handlers
pub async fn create_note(
    State(state): State<AppState>,
    Json(payload): Json<CreateNote>,
) -> Result<Json<ApiResponse<Note>>, StatusCode> {
    match state.note_repo.create(payload).await {
        Ok(note) => Ok(Json(ApiResponse::ok(note))),
        Err(e) => {
            tracing::error!("Failed to create note: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Note>>, StatusCode> {
    match state.note_repo.get_by_id(id).await {
        Ok(Some(note)) => Ok(Json(ApiResponse::ok(note))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get note: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_notes(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Note>>>, StatusCode> {
    match state.note_repo.get_all().await {
        Ok(notes) => Ok(Json(ApiResponse::ok(notes))),
        Err(e) => {
            tracing::error!("Failed to list notes: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateNote>,
) -> Result<Json<ApiResponse<Note>>, StatusCode> {
    match state.note_repo.update(id, payload).await {
        Ok(note) => Ok(Json(ApiResponse::ok(note))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.note_repo.delete(id).await {
        Ok(_) => Ok(Json(ApiResponse::ok(()))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn search_notes(
    State(state): State<AppState>,
    Path(query): Path<String>,
) -> Result<Json<ApiResponse<Vec<Note>>>, StatusCode> {
    match state.note_repo.search(&query).await {
        Ok(notes) => Ok(Json(ApiResponse::ok(notes))),
        Err(e) => {
            tracing::error!("Failed to search notes: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Tag handlers
pub async fn create_tag(
    State(state): State<AppState>,
    Json(payload): Json<CreateTag>,
) -> Result<Json<ApiResponse<Tag>>, StatusCode> {
    match state.tag_repo.create(payload).await {
        Ok(tag) => Ok(Json(ApiResponse::ok(tag))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn list_tags(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Tag>>>, StatusCode> {
    match state.tag_repo.get_all().await {
        Ok(tags) => Ok(Json(ApiResponse::ok(tags))),
        Err(e) => {
            tracing::error!("Failed to list tags: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn add_tag_to_note(
    State(state): State<AppState>,
    Path((note_id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.tag_repo.add_tag_to_note(note_id, tag_id).await {
        Ok(_) => Ok(Json(ApiResponse::ok(()))),
        Err(e) => {
            tracing::error!("Failed to add tag: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn remove_tag_from_note(
    State(state): State<AppState>,
    Path((note_id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.tag_repo.remove_tag_from_note(note_id, tag_id).await {
        Ok(_) => Ok(Json(ApiResponse::ok(()))),
        Err(e) => {
            tracing::error!("Failed to remove tag: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn get_note_tags(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<Tag>>>, StatusCode> {
    match state.tag_repo.get_tags_for_note(note_id).await {
        Ok(tags) => Ok(Json(ApiResponse::ok(tags))),
        Err(e) => {
            tracing::error!("Failed to get tags: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Connection handlers
pub async fn create_connection(
    State(state): State<AppState>,
    Json(payload): Json<CreateConnection>,
) -> Result<Json<ApiResponse<Connection>>, StatusCode> {
    match state.connection_repo.create(payload).await {
        Ok(conn) => Ok(Json(ApiResponse::ok(conn))),
        Err(sqlx::Error::Protocol(_)) => Err(StatusCode::CONFLICT),
        Err(e) => {
            tracing::error!("Failed to create connection: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn list_connections(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Connection>>>, StatusCode> {
    match state.connection_repo.get_all().await {
        Ok(connections) => Ok(Json(ApiResponse::ok(connections))),
        Err(e) => {
            tracing::error!("Failed to list connections: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_note_connections(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<Connection>>>, StatusCode> {
    match state.connection_repo.get_connections_for_note(note_id).await {
        Ok(connections) => Ok(Json(ApiResponse::ok(connections))),
        Err(e) => {
            tracing::error!("Failed to get connections: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_connection(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.connection_repo.delete(id).await {
        Ok(_) => Ok(Json(ApiResponse::ok(()))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Graph data handler (combines notes and connections)
#[derive(Serialize)]
pub struct GraphData {
    nodes: Vec<Note>,
    links: Vec<Connection>,
}

pub async fn get_graph_data(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<GraphData>>, StatusCode> {
    let notes = match state.note_repo.get_all().await {
        Ok(n) => n,
        Err(e) => {
            tracing::error!("Failed to get graph data - notes: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let links = match state.connection_repo.get_all().await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Failed to get graph data - connections: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(ApiResponse::ok(GraphData { nodes: notes, links })))
}

// Suggestion handlers
pub async fn get_suggestions(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<(Uuid, f32)>>>, StatusCode> {
    match state.suggestion_service.suggest_connections_for_note(note_id).await {
        Ok(suggestions) => Ok(Json(ApiResponse::ok(suggestions))),
        Err(e) => {
            tracing::error!("Failed to get suggestions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Health check
pub async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::ok("OK"))
}
