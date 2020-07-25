#![allow(dead_code)]

use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

//this will be refactored that soonish
#[derive(Debug)]
pub enum ErrorKind {
    Empty,
    Diesel(diesel::result::Error),
    DatabaseConnection(r2d2::Error),
    Custom(String),
}

impl Error {
    pub fn is_not_found(&self) -> bool {
        match self.kind {
            ErrorKind::Diesel(diesel::result::Error::NotFound) => true,
            _ => false,
        }
    }

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
            ErrorKind::Diesel(ref inner) => write!(f, "{}", inner),
            ErrorKind::DatabaseConnection(ref inner) => write!(f, "{}", inner),
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

impl From<diesel::result::Error> for ErrorKind {
    fn from(e: diesel::result::Error) -> Self {
        ErrorKind::Diesel(e)
    }
}

impl From<r2d2::Error> for ErrorKind {
    fn from(e: r2d2::Error) -> Self {
        ErrorKind::DatabaseConnection(e)
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