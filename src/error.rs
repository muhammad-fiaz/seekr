use std::path::PathBuf;

use thiserror::Error;

/// The unified error type for all Seekr operations.
#[derive(Error, Debug)]
pub enum SeekrError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("index error: {0}")]
    Index(String),

    #[error("search error: {0}")]
    Search(String),

    #[error("watcher error: {0}")]
    Watcher(String),

    #[error("path not found: {0}")]
    PathNotFound(PathBuf),

    #[error("permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("invalid query: {0}")]
    InvalidQuery(String),

    #[error("cache error: {0}")]
    Cache(String),

    #[error("utf-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
}

/// A specialized `Result` type for Seekr operations.
pub type SeekrResult<T> = Result<T, SeekrError>;
