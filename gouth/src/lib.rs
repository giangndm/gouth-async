//! This library provides auto-renewed tokens for GCP service authentication.
//!
//! # Example
//! ```no_run
//! use gouth::Token;
//! async fn test() {
//!     let token = Token::new().unwrap();
//!     println!("authorization: {}", token.header_value().await.unwrap());
//! }
//! ```

mod error;
mod source;
mod token;

#[cfg(feature = "tonic-intercept")]
pub mod tonic;

pub use error::{Error, ErrorKind, Result};
pub use token::{Builder, Token};
