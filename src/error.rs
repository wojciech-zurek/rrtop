use std::fmt;
use std::num::ParseIntError;
use redis::RedisError;
use flume::{RecvError, SendError};
use crate::event::AppEvent;

#[derive(Debug)]
pub enum RRTopError {
    RedisError(RedisError),
    UnknownQueryRedisError(String),
    ParseIntError(std::num::ParseIntError),
    IoError(std::io::Error),
    RecvError(RecvError),
    SendError(SendError<AppEvent>),
}

impl fmt::Display for RRTopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self)?;
        match self {
            RRTopError::RedisError(e) => write!(f, "{}", e),
            RRTopError::ParseIntError(e) => write!(f, "{}", e),
            RRTopError::UnknownQueryRedisError(e) => write!(f, "{}", e),
            RRTopError::IoError(e) => write!(f, "{}", e),
            RRTopError::RecvError(e) => write!(f, "{}", e),
            RRTopError::SendError(e) => write!(f, "{}", e)
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

impl From<std::io::Error> for RRTopError {
    fn from(e: std::io::Error) -> Self {
        RRTopError::IoError(e)
    }
}

impl From<flume::RecvError> for RRTopError {
    fn from(e: flume::RecvError) -> Self {
        RRTopError::RecvError(e)
    }
}

impl From<flume::SendError<AppEvent>> for RRTopError {
    fn from(e: flume::SendError<AppEvent>) -> Self {
        RRTopError::SendError(e)
    }
}