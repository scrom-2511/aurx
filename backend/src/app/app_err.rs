use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
}

pub type AppResult<T> = Result<T, AppError>;
