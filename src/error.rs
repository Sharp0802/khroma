use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KhromaError {
    #[error("Network or transport error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("API error (status: {status}): {message}")]
    Api {
        status: StatusCode,
        message: String,
    },

    #[error("Failed to parse response: {0}")]
    Parse(String),
}
