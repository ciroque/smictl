use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MemoryLocator {
    pub index: String,
    pub model: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub id: String,
    pub embedding: Vec<f32>,
    pub text: String,
    pub metadata: HashMap<String, String>,
}

pub struct AddEntryParams {
    pub locator: MemoryLocator,
    pub entry: MemoryEntry,
}

pub struct QueryParams {
    pub locator: MemoryLocator,
    pub embedding: Vec<f32>,
    pub top_k: usize,
    pub filters: Option<HashMap<String, String>>,
}

pub struct IndexParams {
    pub name: String,
    pub model: String,
}
