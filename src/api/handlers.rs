use axum::{Json, extract::State};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::db::memory::MemoryDB;

#[derive(Debug, Deserialize)]
pub struct InsertRequest {
    pub vector: Vec<f32>,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InsertResponse {
    pub id: Uuid,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: Vec<f32>,
    pub top_k: usize,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub score: f32,
    pub metadata: Option<String>,
}

pub type AppState = Arc<Mutex<MemoryDB>>;

pub async fn insert_vector(
    State(state): State<AppState>,
    Json(payload): Json<InsertRequest>,
) -> Json<InsertResponse> {
    let mut db = state.lock().unwrap();
    let id = db.insert(payload.vector, payload.metadata);
    Json(InsertResponse { id, status: "ok".to_string() })
}

pub async fn search_vectors(
    State(state): State<AppState>,
    Json(payload): Json<SearchRequest>,
) -> Json<Vec<SearchResult>> {
    let db = state.lock().unwrap();
    let scored = db.search(payload.query, payload.top_k);

    // ⬇️ Explicitly destructure the tuple (score, entry)
    let results: Vec<SearchResult> = scored
        .into_iter()
        .map(|(score, entry)| SearchResult {
            id: entry.id,
            score,
            metadata: entry.metadata,
        })
        .collect();

    Json(results)
}

