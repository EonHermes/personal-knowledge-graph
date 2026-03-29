//! Personal Knowledge Graph Backend
//! 
//! A Rust-based REST API for managing a personal knowledge graph
//! with notes, bookmarks, tags, and connections.

pub mod api;
pub mod db;
pub mod models;
pub mod services;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_compiles() {
        // Basic compilation test
        assert!(true);
    }
}
