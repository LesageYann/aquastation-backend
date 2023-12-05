pub mod error;
pub mod auth;
pub mod user;

use crate::api::error::AQSError;

pub type AQSRequest<T> = Result<T, AQSError>;
