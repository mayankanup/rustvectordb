use axum::{Router, routing::{post}};
use std::sync::{Arc, Mutex};
use crate::api::handlers::{insert_vector, search_vectors};
use crate::db::memory::MemoryDB;

pub fn create_router() -> Router {
    let state = Arc::new(Mutex::new(MemoryDB::new()));

    Router::new()
        .route("/insert", post(insert_vector))
        .route("/search", post(search_vectors))
        .with_state(state)
}
