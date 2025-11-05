use redis::{AsyncCommands, RedisResult};

use crate::services::ipfs_uploader::IPFSUploader;

pub struct RedisTaskQueue {
    connection: redis::aio::Connection,
    queue_name: String,
    uploader: IPFSUploader
}

impl RedisTaskQueue {
    pub fn new(connection: redis::aio::Connection, queue_name: &str, uploader: IPFSUploader) -> Self {
        Self {
            connection,
            queue_name: queue_name.to_string(),
            uploader
        }
    }

    pub async fn push_task(&mut self, data: &str) -> RedisResult<()> {
        self.connection.lpush(&self.queue_name, data).await
    }

    pub async fn pop_task(&mut self) -> RedisResult<()> {
        let result:Option<(String, String)> = self.connection.brpop(&self.queue_name, 0.0).await?;

        if let Some((_, chunk)) = result {
            match self.uploader.upload_chunk(chunk).await{
                Ok(_) => print!("Chunks uploaded Successfully!"),
                Err(_) => print!("There was an error")
            }
        }
        Ok(())
    }
}
