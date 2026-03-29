mod api;
mod db;
mod models;
mod services;

use api::routes::create_router;
use api::handlers::AppState;
use db::{create_pool, migrate_database, repositories::{ConnectionRepository, NoteRepository, TagRepository}};
use services::SuggestionService;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "personal_knowledge_graph_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database setup
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:personal_knowledge_graph.db".to_string());
    
    let pool = create_pool(&database_url).await?;
    migrate_database(&pool).await?;

    // Create repositories
    let note_repo = Arc::new(NoteRepository::new(pool.clone()));
    let tag_repo = Arc::new(TagRepository::new(pool.clone()));
    let connection_repo = Arc::new(ConnectionRepository::new(pool.clone()));
    let suggestion_service = Arc::new(SuggestionService::new(pool));

    // Create application state
    let app_state = AppState {
        note_repo,
        tag_repo,
        connection_repo,
        suggestion_service,
    };

    // Create router
    let app = create_router(app_state)
        .layer(TraceLayer::new_for_http());

    // Get port from environment or use default
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
