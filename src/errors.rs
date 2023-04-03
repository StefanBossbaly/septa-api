use std::fmt;

use crate::responses::ApiError;

#[derive(Debug)]
pub enum Error {
    RequestFailed(reqwest::Error),
    DeserializeFailed(serde_json::error::Error),
    ApiErrorResponse(ApiError),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::RequestFailed(e) => {
                write!(f, "Unable to send the request: {}", e)
            }
            Self::DeserializeFailed(e) => {
                write!(f, "Unable to deserialize the received value: {}", e)
            }
            Self::ApiErrorResponse(e) => {
                write!(f, "API returned an error response: {}", e.error)
            }
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Self::DeserializeFailed(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestFailed(error)
    }
}

impl From<ApiError> for Error {
    fn from(error: ApiError) -> Self {
        Self::ApiErrorResponse(error)
    }
}
