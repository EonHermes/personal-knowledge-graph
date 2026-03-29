use crate::api::handlers::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Notes routes
        .route("/api/notes", post(create_note).get(list_notes))
        .route("/api/notes/:id", get(get_note).put(update_note).delete(delete_note))
        .route("/api/notes/search/:query", get(search_notes))
        
        // Tags routes
        .route("/api/tags", post(create_tag).get(list_tags))
        .route("/api/notes/:note_id/tags", get(get_note_tags))
        .route("/api/notes/:note_id/tags/:tag_id", post(add_tag_to_note).delete(remove_tag_from_note))
        
        // Connections routes
        .route("/api/connections", post(create_connection).get(list_connections))
        .route("/api/notes/:note_id/connections", get(get_note_connections))
        .route("/api/connections/:id", delete(delete_connection))
        
        // Graph data
        .route("/api/graph", get(get_graph_data))
        
        // Suggestions
        .route("/api/notes/:note_id/suggestions", get(get_suggestions))
        
        // Apply state to all routes
        .with_state(state)
}
