use std::fmt;
use std::fmt::{Debug, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

use log4rs::config::runtime::ConfigErrors;
use log::SetLoggerError;
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
    LoggerError,
}

pub struct AppError {
    message: String,
    kind: ErrorKind,
}

impl AppError {
    pub fn cli_parse_error(message: String) -> Self {
        AppError {
            message,
            kind: ErrorKind::CliParseError,
        }
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} -> {}", self.kind, self.message)
    }
}

impl std::error::Error for AppError {}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError {
            message: format!("{}", e),
            kind: ErrorKind::ParseIntError,
        }
    }
}

impl From<ParseFloatError> for AppError {
    fn from(e: ParseFloatError) -> Self {
        AppError {
            message: format!("{}", e),
            kind: ErrorKind::ParseFloatError,
        }
    }
}

impl From<ParseBoolError> for AppError {
    fn from(e: ParseBoolError) -> Self {
        AppError {
            message: format!("{}", e),
            kind: ErrorKind::ParseBoolError,
        }
    }
}

impl From<RedisError> for AppError {
    fn from(e: RedisError) -> Self {
        AppError {
            message: format!("{}", e),
            kind: ErrorKind::RedisError,
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(e: r2d2::Error) -> Self {
        AppError {
            message: format!("{}", e),
            kind: ErrorKind::RedisPoolError,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError {
            message: format!("{}", e),
            kind: ErrorKind::IoError,
        }
    }
}

impl From<flume::RecvError> for AppError {
    fn from(e: flume::RecvError) -> Self {
        AppError {
            message: format!("{}", e),
            kind: ErrorKind::RecvError,
        }
    }
}

impl From<flume::SendError<AppEvent>> for AppError {
    fn from(e: flume::SendError<AppEvent>) -> Self {
        AppError {
            message: format!("{}", e),
            kind: ErrorKind::SendError,
        }
    }
}

impl From<ConfigErrors> for AppError {
    fn from(e: ConfigErrors) -> Self {
        AppError {
            message: format!("{}", e.errors().iter().map(|it| format!("{:?}", it)).collect::<String>()),
            kind: ErrorKind::LoggerError,
        }
    }
}

impl From<SetLoggerError> for AppError {
    fn from(e: SetLoggerError) -> Self {
        AppError {
            message: format!("{}", e),
            kind: ErrorKind::LoggerError,
        }
    }
}