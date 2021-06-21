use std::fmt;
use std::num::ParseIntError;
use redis::RedisError;

#[derive(Debug)]
pub enum RRTopError {
    RedisError(RedisError),
    UnknownQueryRedisError(String),
    ParseIntError(std::num::ParseIntError),
}

impl fmt::Display for RRTopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self)?;
        match self {
            RRTopError::RedisError(e) => write!(f, "{}", e),
            RRTopError::ParseIntError(e) => write!(f, "{}", e),
            RRTopError::UnknownQueryRedisError(e) => write!(f, "{}", e)
        }
    }
}

impl std::error::Error for RRTopError {}

impl From<ParseIntError> for RRTopError {
    fn from(e: ParseIntError) -> Self {
        RRTopError::ParseIntError(e)
    }
}

impl From<RedisError> for RRTopError {
    fn from(e: RedisError) -> Self {
        RRTopError::RedisError(e)
    }
}