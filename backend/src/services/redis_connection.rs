pub struct RedisConnection {
    client: redis::Client,
}

impl RedisConnection {
    pub fn new(redis_url: String) -> redis::RedisResult<Self> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self {client})
    }
    
    pub async fn get_handler_and_worker_connection(&self) -> redis::RedisResult<(redis::aio::MultiplexedConnection, redis::aio::MultiplexedConnection)>{
        let handler_conn = self.client.get_multiplexed_async_connection().await?;
        let worker_conn = self.client.get_multiplexed_async_connection().await?;

        Ok((handler_conn, worker_conn))
    }
}
