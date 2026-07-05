#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RepositoryError {
    #[error("repository validation failed: {0}")]
    Validation(String),
    #[error("generation not found")]
    NotFound,
    #[error("repository conflict: {0}")]
    Conflict(String),
    #[error("database error: {0}")]
    Database(String),
    #[error("serialization error: {0}")]
    Serialization(String),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error.to_string())
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error.to_string())
    }
}
