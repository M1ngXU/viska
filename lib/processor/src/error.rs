#![allow(dead_code)]

use common::libsip;
use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

//this will be refactored that soonish
#[derive(Debug)]
pub enum ErrorKind {
    Empty,
    Models(models::Error),
    Store(store::Error),
    Libsip(String),
    Custom(String),
}

impl Error {
    pub fn custom(reason: String) -> Self {
        Self {
            kind: ErrorKind::from(reason),
        }
    }
}

impl From<Option<ErrorKind>> for ErrorKind {
    fn from(kind: Option<ErrorKind>) -> Self {
        match kind {
            None => ErrorKind::Empty,
            Some(kind) => kind,
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::Models(ref inner) => write!(f, "models transformation error: {}", inner),
            ErrorKind::Store(ref inner) => write!(f, "store error: {}", inner),
            ErrorKind::Custom(ref inner) => write!(f, "{}", inner),
            _ => write!(f, "unknown error, {:?}", self),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl StdError for Error {}

impl<E> From<E> for Error
where
    E: Into<ErrorKind>,
{
    fn from(e: E) -> Self {
        Error { kind: e.into() }
    }
}

impl From<String> for ErrorKind {
    fn from(e: String) -> Self {
        ErrorKind::Custom(e)
    }
}

impl From<&str> for ErrorKind {
    fn from(e: &str) -> Self {
        ErrorKind::Custom(e.into())
    }
}

impl From<models::Error> for ErrorKind {
    fn from(e: models::Error) -> Self {
        ErrorKind::Models(e)
    }
}

impl From<store::Error> for ErrorKind {
    fn from(e: store::Error) -> Self {
        ErrorKind::Store(e)
    }
}

impl From<libsip::core::MissingUsernameError> for ErrorKind {
    fn from(e: libsip::core::MissingUsernameError) -> Self {
        ErrorKind::Libsip(format!("{:?}", e))
    }
}

impl From<libsip::core::MissingHeaderError> for ErrorKind {
    fn from(e: libsip::core::MissingHeaderError) -> Self {
        ErrorKind::Libsip(format!("{:?}", e))
    }
}

impl From<libsip::core::MissingViaBranchError> for ErrorKind {
    fn from(e: libsip::core::MissingViaBranchError) -> Self {
        ErrorKind::Libsip(format!("{:?}", e))
    }
}