#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request error {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Deserialization error {0}")]
    ParseJsonError(#[from] serde_json::Error),
    #[error("API error({code}): {message}")]
    APIError { code: i32, message: String },
    #[error("Other error: {0}")]
    Other(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type APIResult<T> = Result<T, Error>;

