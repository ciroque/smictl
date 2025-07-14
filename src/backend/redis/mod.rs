use crate::backend::{Backend, BackendError};
use crate::backend::types::{AddEntryParams, QueryParams, IndexParams, MemoryEntry};
use async_trait::async_trait;
use super::redis::client::RedisClient;

pub mod client;

pub struct RedisBackend {
    client: RedisClient,
    connection_url: String,
}

impl RedisBackend {
    pub async fn new() -> Result<Self, BackendError> {
        let connection_url = "redis://127.0.0.1:6379/".to_string();
        let client = RedisClient::new(&connection_url)
            .await
            .map_err(|e| BackendError::Internal(e.to_string()))?;
        Ok(Self { client, connection_url })
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

    async fn create_index(&self, index_params: IndexParams) -> Result<(), BackendError> {
        self.client
            .create_index(&index_params)
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
    
    fn get_connection_info(&self) -> String {
        self.connection_url.clone()
    }
}
