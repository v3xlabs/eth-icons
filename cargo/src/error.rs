#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Internal error: {0}")]
    InternalError(String),
}
