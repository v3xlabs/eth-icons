#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Response body exceeds the {max_bytes}-byte limit")]
    ResponseTooLarge { max_bytes: usize },
    #[error("Internal error: {0}")]
    InternalError(String),
}
