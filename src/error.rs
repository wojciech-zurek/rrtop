use std::fmt;
use std::fmt::{Debug, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

use flume::{RecvError, SendError};
use redis::RedisError;

use crate::event::AppEvent;

pub enum RRTopError {
    RedisError(RedisError),
    RedisPoolError(r2d2::Error),
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    ParseBoolError(std::str::ParseBoolError),
    IoError(std::io::Error),
    RecvError(RecvError),
    SendError(SendError<AppEvent>),
    CliParseError(String),
}

impl Debug for RRTopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for RRTopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RRTopError::RedisError(e) => write!(f, "{}", e),
            RRTopError::RedisPoolError(e) => { write!(f, "{}", e) }
            RRTopError::ParseIntError(e) => write!(f, "{}", e),
            RRTopError::ParseFloatError(e) => write!(f, "{}", e),
            RRTopError::ParseBoolError(e) => write!(f, "{}", e),
            RRTopError::IoError(e) => write!(f, "{}", e),
            RRTopError::RecvError(e) => write!(f, "{}", e),
            RRTopError::SendError(e) => write!(f, "{}", e),
            RRTopError::CliParseError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for RRTopError {}

impl From<ParseIntError> for RRTopError {
    fn from(e: ParseIntError) -> Self {
        RRTopError::ParseIntError(e)
    }
}

impl From<ParseFloatError> for RRTopError {
    fn from(e: ParseFloatError) -> Self {
        RRTopError::ParseFloatError(e)
    }
}

impl From<ParseBoolError> for RRTopError {
    fn from(e: ParseBoolError) -> Self {
        RRTopError::ParseBoolError(e)
    }
}

impl From<RedisError> for RRTopError {
    fn from(e: RedisError) -> Self {
        RRTopError::RedisError(e)
    }
}

impl From<r2d2::Error> for RRTopError {
    fn from(e: r2d2::Error) -> Self {
        RRTopError::RedisPoolError(e)
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