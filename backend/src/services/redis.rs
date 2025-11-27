use redis::{AsyncCommands, RedisError, RedisResult};

#[derive(Clone)]
pub struct RedisTaskQueue {
    connection: redis::aio::MultiplexedConnection,
    queue_name: String
}

impl RedisTaskQueue {
    pub fn new(connection: redis::aio::MultiplexedConnection, queue_name: &str) -> Self {
        Self {
            connection: connection,
            queue_name: queue_name.to_string(),
        }
    }

    pub async fn push_task(&self, data: &str) -> RedisResult<()> {
        let mut conn = self.connection.clone();
        conn.lpush(&self.queue_name, data).await
    }

    pub async fn pop_task(&self) -> Result<(String, String), RedisError> {
        let mut conn = self.connection.clone();
        let result = conn.brpop(&self.queue_name, 0.0).await?;
        println!("{:?}", result);
        Ok(result)
    }
}