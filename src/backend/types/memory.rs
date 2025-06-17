use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub id: String,
    pub embedding: Vec<f32>,
    pub text: String,
    pub metadata: HashMap<String, String>,
}
