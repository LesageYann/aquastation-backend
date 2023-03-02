use axum::http::StatusCode;
use axum::{Json};
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub enum ResponseContent<T>  where T: Serialize {
    Ok{data: Json<T>},
    Err{error: Option<String>}
}

pub struct ServerResponse<T>  where T: Serialize{
    content: ResponseContent<T>
}

impl<T> ServerResponse<T>  where T: Serialize {
    pub fn ok(data: T) -> ServerResponse<T> {
        ServerResponse { content: ResponseContent::Ok { data: Json(data) } }
    }
}

impl<T> IntoResponse for ServerResponse<T>  where T: Serialize{
    fn into_response(self) -> Response {
        match self.content {
            ResponseContent::Ok { data } => {
                to_response(StatusCode::OK, data)
            }
            ResponseContent::Err { error } => {
                to_response(StatusCode::INTERNAL_SERVER_ERROR,error.unwrap_or("".to_owned()))
                // TODO: use error to change the status code
            }
        }
    }
}

fn to_response<U>( status: StatusCode, data: U) -> Response where U: IntoResponse {
    let mut res = data.into_response();
    *res.status_mut() = status;
    res
}