use async_trait::async_trait;

use super::types::{AddEntryParams, QueryParams, IndexParams, MemoryEntry};

#[derive(Debug)]
pub enum BackendError {
    NotFound,
    InvalidInput(String),
    TransportError(String),
    Internal(String),
}

#[async_trait]
pub trait Backend: Send + Sync {
    fn name(&self) -> &'static str;

    async fn add_entry(&self, params: AddEntryParams) -> Result<String, BackendError>;

    async fn query(&self, params: QueryParams) -> Result<Vec<MemoryEntry>, BackendError>;

    async fn list_indexes(&self) -> Result<Vec<String>, BackendError>;

    async fn create_index(&self, params: IndexParams) -> Result<(), BackendError>;

    async fn delete_index(&self, name: &str) -> Result<(), BackendError>;
}
