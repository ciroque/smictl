/* ---------------------------------------------------------------
 * src/backend/redis/client.rs
----------------------------------------------------------------*/

use redis::aio::MultiplexedConnection;
use redis::{Client, RedisError};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::backend::types::{IndexParams, MemoryEntry, AddEntryParams, QueryParams};

#[derive(Clone)]
pub struct RedisClient {
    connection: Arc<Mutex<MultiplexedConnection>>,
}

impl RedisClient {
    pub async fn new(url: &str) -> Result<Self, RedisError> {
        let client = Client::open(url)?;
        let connection = client.get_multiplexed_tokio_connection().await?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub async fn list_indexes(&self) -> Result<Vec<String>, RedisError> {
        let mut conn = self.connection.lock().await;
        let indexes: Vec<String> = redis::cmd("FT._LIST").query_async(&mut *conn).await?;
        Ok(indexes)
    }


    // backend select n
    // backend index create <prefix> <model> <version> --dim 768 --vector-type FLAT --distance L2 --schema embedding:vector,text:text,model:tag,source:tag

    pub async fn create_index(&self, params: &IndexParams) -> Result<(), RedisError> {
        let mut conn = self.connection.lock().await;

        let args = params.to_redis_args();
        
        redis::cmd("FT.CREATE")
            .arg(args)
            .query_async(&mut *conn)
            .await
    }

    pub async fn add_entry(&self, _params: &AddEntryParams) -> Result<String, RedisError> {
        Err(RedisError::from((redis::ErrorKind::IoError, "not implemented")))
    }

    pub async fn query(&self, _params: &QueryParams) -> Result<Vec<MemoryEntry>, RedisError> {
        Err(RedisError::from((redis::ErrorKind::IoError, "not implemented")))
    }

    pub async fn delete_index(&self, name: &str) -> Result<(), RedisError> {
        let mut conn = self.connection.lock().await;
        redis::cmd("FT.DROPINDEX")
            .arg(name)
            .arg("DD") // DD = Drop the underlying hash data too
            .query_async(&mut *conn)
            .await
    }
}
