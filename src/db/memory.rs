use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::utils::similarity::cosine_similarity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VectorEntry {
    pub id: Uuid,
    pub vector: Vec<f32>,
    pub metadata: Option<String>,
}

pub struct MemoryDB {
    pub vectors: Vec<VectorEntry>,
}

impl MemoryDB {
    pub fn new() -> Self {
        Self { vectors: Vec::new() }
    }

    pub fn insert(&mut self, vector: Vec<f32>, metadata: Option<String>) -> Uuid {
        let id = Uuid::new_v4();
        self.vectors.push(VectorEntry { id, vector, metadata });
        id
    }

    pub fn search(&self, query: Vec<f32>, top_k: usize) -> Vec<(f32, VectorEntry)> {
    let mut results: Vec<(f32, VectorEntry)> = self.vectors
        .iter()
        .cloned()
        .map(|entry| {
            let score = cosine_similarity(&query, &entry.vector);
            (score, entry)
        })
        .collect();

    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    results.into_iter().take(top_k).collect()
}

}
