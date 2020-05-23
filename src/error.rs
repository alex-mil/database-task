use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error("Unknown command: {0}")]
    InvalidCommand(String),
    #[error("{0}")]
    InvalidDate(String),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError)
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;