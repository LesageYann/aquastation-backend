use std::fmt;
use std::fmt::{Display, Formatter};
use leptos::server_fn::serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum AQSError {
    InvalidData,
    NetworkError
}

impl Display for AQSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AQSError::InvalidData => write!(f, "invalid_data"),
            AQSError::NetworkError => write!(f, "network_error"),
        }
    }
}