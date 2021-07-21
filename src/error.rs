use std::fmt;
use std::fmt::{Debug, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

use redis::RedisError;

use crate::event::AppEvent;

#[derive(Debug)]
pub enum ErrorKind {
    RedisError,
    RedisPoolError,
    ParseIntError,
    ParseFloatError,
    ParseBoolError,
    IoError,
    RecvError,
    SendError,
    CliParseError,
}

pub struct RRTopError {
    message: String,
    kind: ErrorKind,
}

impl RRTopError {
    pub fn cli_parse_error(message: String) -> Self {
        RRTopError {
            message,
            kind: ErrorKind::CliParseError,
        }
    }
}

impl Debug for RRTopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for RRTopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} -> {}", self.kind, self.message)
    }
}

impl std::error::Error for RRTopError {}

impl From<ParseIntError> for RRTopError {
    fn from(e: ParseIntError) -> Self {
        RRTopError {
            message: format!("{}", e),
            kind: ErrorKind::ParseIntError,
        }
    }
}

impl From<ParseFloatError> for RRTopError {
    fn from(e: ParseFloatError) -> Self {
        RRTopError {
            message: format!("{}", e),
            kind: ErrorKind::ParseFloatError,
        }
    }
}

impl From<ParseBoolError> for RRTopError {
    fn from(e: ParseBoolError) -> Self {
        RRTopError {
            message: format!("{}", e),
            kind: ErrorKind::ParseBoolError,
        }
    }
}

impl From<RedisError> for RRTopError {
    fn from(e: RedisError) -> Self {
        RRTopError {
            message: format!("{}", e),
            kind: ErrorKind::RedisError,
        }
    }
}

impl From<r2d2::Error> for RRTopError {
    fn from(e: r2d2::Error) -> Self {
        RRTopError {
            message: format!("{}", e),
            kind: ErrorKind::RedisPoolError,
        }
    }
}

impl From<std::io::Error> for RRTopError {
    fn from(e: std::io::Error) -> Self {
        RRTopError {
            message: format!("{}", e),
            kind: ErrorKind::IoError,
        }
    }
}

impl From<flume::RecvError> for RRTopError {
    fn from(e: flume::RecvError) -> Self {
        RRTopError {
            message: format!("{}", e),
            kind: ErrorKind::RecvError,
        }
    }
}

impl From<flume::SendError<AppEvent>> for RRTopError {
    fn from(e: flume::SendError<AppEvent>) -> Self {
        RRTopError {
            message: format!("{}", e),
            kind: ErrorKind::SendError,
        }
    }
}