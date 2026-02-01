use thiserror::Error;

#[derive(Debug, Error)]
pub enum ItaError {
    #[error("API error: {message} (type: {error_type})")]
    Api { message: String, error_type: String },

    #[error("transport: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("unexpected status: outer={outer}, inner={inner}")]
    UnexpectedStatus { outer: u16, inner: u16 },

    #[error("failed to parse batch response: {reason}")]
    BatchParse { reason: &'static str },

    #[error("missing field in response: {field}")]
    MissingField { field: &'static str },
}

pub type Result<T> = std::result::Result<T, ItaError>;
