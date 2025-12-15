use core::fmt;
use eventsource_stream::EventStreamError;
use nom::error::Error as NomError;
use reqwest::header::HeaderValue;
use reqwest::Error as ReqwestError;
use reqwest::Response;
use reqwest::StatusCode;
use std::string::FromUtf8Error;

#[cfg(doc)]
use reqwest::RequestBuilder;

/// Error raised when a [`RequestBuilder`] cannot be created
#[derive(Debug, Clone)]
pub struct CannotCreateRequestBuilderError(String);

impl CannotCreateRequestBuilderError {
    pub fn new(error: String) -> Self {
        Self(error)
    }
}

impl fmt::Display for CannotCreateRequestBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CannotCreateRequestBuilderError {}

/// Error raised by the EventSource stream fetching and parsing
#[derive(Debug, Error)]
pub enum Error {
    /// Source stream is not valid UTF8
    #[error(transparent)]
    Utf8(FromUtf8Error),
    /// Source stream is not a valid EventStream
    #[error(transparent)]
    Parser(NomError<String>),
    /// The HTTP Request could not be completed
    #[error(transparent)]
    Transport(ReqwestError),
    /// The `Content-Type` returned by the server is invalid
    #[error("Invalid header value: {0:?}")]
    InvalidContentType(HeaderValue, Box<Response>),
    /// The status code returned by the server is invalid
    #[error("Invalid status code: {0}")]
    InvalidStatusCode(StatusCode, Box<Response>),
    /// The `Last-Event-ID` cannot be formed into a Header to be submitted to the server
    #[error("Invalid `Last-Event-ID`: {0}")]
    InvalidLastEventId(String),
    /// The stream ended
    #[error("Stream ended")]
    StreamEnded,
    /// The RequestBuilder could not be created
    #[error(transparent)]
    CannotCreateRequestBuilder(CannotCreateRequestBuilderError),
}

impl From<EventStreamError<ReqwestError>> for Error {
    fn from(err: EventStreamError<ReqwestError>) -> Self {
        match err {
            EventStreamError::Utf8(err) => Self::Utf8(err),
            EventStreamError::Parser(err) => Self::Parser(err),
            EventStreamError::Transport(err) => Self::Transport(err),
        }
    }
}

impl From<CannotCreateRequestBuilderError> for Error {
    fn from(err: CannotCreateRequestBuilderError) -> Self {
        Self::CannotCreateRequestBuilder(err)
    }
}
