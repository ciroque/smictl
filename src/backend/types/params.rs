use std::collections::HashMap;

use super::locator::MemoryLocator;
use super::memory::MemoryEntry;
use super::schema::SchemaField;

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
    pub schema: Vec<SchemaField>,
}

impl IndexParams {
    pub fn to_redis_args(&self) -> Vec<String> {
        let mut args = vec![
            self.name.clone(),
            "ON".into(), "HASH".into(),
            "PREFIX".into(), "1".into(), "mem:".into(),
            "SCHEMA".into(),
        ];

        for field in &self.schema {
            args.extend(field.to_redis_args());
        }

        args
    }
}
