use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use crate::model::error::ASError;

impl IntoResponse for ASError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ASError::InvalidToken | ASError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string()),
            ASError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials".to_string()),
            ASError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error".to_string()),
            ASError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            ASError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            ASError::InternalError => {(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())}
            ASError::InvalidRequest(reasons) => (StatusCode::BAD_REQUEST, reasons.join(",")),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
