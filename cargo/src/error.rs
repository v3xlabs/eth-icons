#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("HTTP error: {0}")]
    HttpError(reqwest::Error),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Unsupported")]
    Unsupported,
}
