use std::{convert::From, fmt};

/// Represents the details of the [`Error`](struct.Error.html)
#[derive(Debug)]
pub enum ErrorKind {
    /// Errors that can possibly occur while accessing an HTTP server.
    Http(surf::Error),
    /// Http status code that is not 2xx when getting token.
    HttpStatus(surf::StatusCode),
    /// GCE metadata service error.
    Metadata(gcemeta::Error),
    /// JWT encode/decode error.
    Jwt(jsonwebtoken::errors::Error),
    /// Token source error.
    TokenSource,
    /// An error parsing credentials file.
    CredentialsJson(serde_json::Error),
    /// An error reading credentials file.
    CredentialsFile(std::io::Error),
    /// An error parsing data from token response.
    TokenJson(serde_json::Error),
    /// Invalid token error.
    TokenData,
    #[doc(hidden)]
    __Nonexhaustive,
}

/// Represents errors that can occur during getting token.
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// Borrow [`ErrorKind`](enum.ErrorKind.html).
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    /// To own [`ErrorKind`](enum.ErrorKind.html).
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match *self.0 {
            Http(ref e) => write!(f, "http error: {}", e),
            HttpStatus(ref s) => write!(f, "http status error: {}", s),
            Metadata(ref e) => write!(f, "gce metadata service error: {}", e),
            Jwt(ref e) => write!(f, "jwt error: {}", e),
            TokenSource => write!(f, "token source error: not found token source"),
            CredentialsJson(ref e) => write!(f, "credentials json error: {}", e),
            CredentialsFile(ref e) => write!(f, "credentials file error: {}", e),
            TokenJson(ref e) => write!(f, "token json error: {}", e),
            TokenData => write!(f, "token data error: invalid token response data"),
            __Nonexhaustive => write!(f, "unknown error"),
        }
    }
}

impl std::error::Error for Error {}

impl From<surf::Error> for Error {
    fn from(e: surf::Error) -> Self {
        ErrorKind::Http(e).into()
    }
}

impl From<gcemeta::Error> for Error {
    fn from(e: gcemeta::Error) -> Self {
        ErrorKind::Metadata(e).into()
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        ErrorKind::Jwt(e).into()
    }
}

impl From<ErrorKind> for Error {
    fn from(k: ErrorKind) -> Self {
        Error(Box::new(k))
    }
}

/// Wrapper for the `Result` type with an [`Error`](struct.Error.html).
pub type Result<T> = std::result::Result<T, Error>;
