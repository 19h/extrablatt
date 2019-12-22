use bytes::Bytes;
use thiserror::Error;

/// All different error types this crate uses.
#[derive(Error, Debug)]
pub enum ExtrablattError {
    /// Received a good non success Http response
    #[error("Expected a 2xx Success but got: {}", response.status())]
    NoHttpSuccessResponse {
        /// The good reqwest response.
        response: reqwest::Response,
    },
    /// Failed to get a response.
    #[error("Request failed: {error}")]
    HttpRequestFailure {
        /// The reqwest error.
        error: reqwest::Error,
    },
    /// Failed to read a document.
    #[error("Failed to read document")]
    ReadDocumentError {
        /// The content the resulted in the error.
        body: Bytes,
    },
}
