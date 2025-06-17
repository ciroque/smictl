use crate::backend::{Backend, BackendError, AddEntryParams, QueryParams, IndexParams, MemoryEntry};
use async_trait::async_trait;
use super::redis::client::RedisClient;

pub mod client;

pub struct RedisBackend {
    client: RedisClient,
}

impl RedisBackend {
    pub async fn new() -> Result<Self, BackendError> {
        let client = RedisClient::new("redis://127.0.0.1:6379/")
            .await
            .map_err(|e| BackendError::Internal(e.to_string()))?;
        Ok(Self { client })
    }
}

#[async_trait]
impl Backend for RedisBackend {
    fn name(&self) -> &'static str {
        "redis"
    }

    async fn add_entry(&self, _params: AddEntryParams) -> Result<String, BackendError> {
        Err(BackendError::Internal("Not implemented".into()))
    }

    async fn create_index(&self, indexParams: IndexParams) -> Result<(), BackendError> {
        self.client
            .create_index(&indexParams)
            .await
            .map_err(|e| BackendError::Internal(e.to_string()))
    }

    async fn delete_index(&self, name: &str) -> Result<(), BackendError> {
        self.client
            .delete_index(name)
            .await
            .map_err(|e| BackendError::Internal(e.to_string()))
    }

    async fn list_indexes(&self) -> Result<Vec<String>, BackendError> {
        self.client
            .list_indexes()
            .await
            .map_err(|e| BackendError::Internal(e.to_string()))
    }

    async fn query(&self, _params: QueryParams) -> Result<Vec<MemoryEntry>, BackendError> {
        Err(BackendError::Internal("Not implemented".into()))
    }
}
