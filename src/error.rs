use core::fmt;
use std::{error::Error, fmt::Display};

use reqwest::StatusCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeatimeErrorKind {
    AuthError,
    RepoCreateError,
}

impl Display for TeatimeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TeatimeErrorKind::AuthError => write!(f, "AuthError"),
            TeatimeErrorKind::RepoCreateError => write!(f, "RepoCreateError"),
        }
    }
}

/// Represents some kind of error that can occur when interacting with the Gitea API.
/// This simply wraps a message and a status code.
#[derive(Debug, Clone)]
pub struct TeatimeError {
    pub message: String,
    pub status_code: reqwest::StatusCode,
}
impl Error for TeatimeError {}
impl Display for TeatimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// A type alias for a [std::result::Result] that uses [TeatimeError] as the error type.
/// We define this purely for convenience.
pub type Result<T> = std::result::Result<T, TeatimeError>;

/// Converts a [reqwest::Error] into a [TeatimeError].
/// This method exists for us to be able to directly call the unwrap operator (`?`) on the result
/// of a [reqwest::Result].
impl From<reqwest::Error> for TeatimeError {
    fn from(err: reqwest::Error) -> Self {
        TeatimeError {
            message: format!("{}", err),
            status_code: err.status().unwrap_or(StatusCode::BAD_REQUEST),
        }
    }
}

