use thiserror::Error;

pub type Result<T> = std::result::Result<T, WcError>;

#[derive(Debug, Error)]
pub enum WcError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("config error: {0}")]
    Config(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("ai error: {0}")]
    Ai(String),
}
