use reqwest::StatusCode;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// The error returned by API calls
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The access token is missing, invalid, expired or lacks the required scope (HTTP 401)
    #[error("unauthorized: {body}")]
    Unauthorized { body: String },

    /// The Strava rate limit has been exceeded (HTTP 429)
    #[error("rate limit exceeded: {body}")]
    RateLimited { body: String },

    /// Any other non-success response
    #[error("unexpected status {status}: {body}")]
    Status { status: u16, body: String },

    /// The request could not be sent or the response could not be read
    #[error("request failed: {0}")]
    Request(#[source] BoxError),

    /// The response body did not match the expected model
    #[error("failed to decode response: {0}")]
    Decode(#[source] BoxError),
}

impl Error {
    pub(crate) fn from_status(status: StatusCode, body: String) -> Error {
        match status {
            StatusCode::UNAUTHORIZED => Error::Unauthorized { body },
            StatusCode::TOO_MANY_REQUESTS => Error::RateLimited { body },
            _ => Error::Status {
                status: status.as_u16(),
                body,
            },
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Request(Box::new(error))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Decode(Box::new(error))
    }
}
