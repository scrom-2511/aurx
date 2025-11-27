use redis::{AsyncCommands, RedisError, RedisResult, aio::MultiplexedConnection};

#[derive(Clone)]
pub struct RedisTaskQueue {
    conn_for_push: redis::aio::MultiplexedConnection,
    conn_for_pop: redis::aio::MultiplexedConnection,
    queue_name: String
}

impl RedisTaskQueue {
    pub fn new(connections: (MultiplexedConnection, MultiplexedConnection), queue_name: &str) -> Self {
        Self {
            conn_for_push: connections.0,
            conn_for_pop: connections.1,
            queue_name: queue_name.to_string(),
        }
    }

    pub async fn push_task(&self, data: &str) -> RedisResult<()> {
        let mut conn = self.conn_for_push.clone();
        conn.lpush(&self.queue_name, data).await
    }

    pub async fn pop_task(&self) -> Result<(String, String), RedisError> {
        let mut conn = self.conn_for_pop.clone();
        let result = conn.brpop(&self.queue_name, 0.0).await?;
        println!("{:?}", result);
        Ok(result)
    }
}