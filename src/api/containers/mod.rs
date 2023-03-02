use axum::{Form, Router};
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};

use crate::api::app_state::AppState;
use crate::api::helpers::server_response::ServerResponse;
use crate::usecases::containers::get::{GetContainerData, get_container_usecase};
use crate::usecases::containers::list::{ListContainerData, list_containers_usecase};

pub fn build_containers_router() -> Router<AppState, Body> {
    Router::new()
        .route("/SignIn", post(handle_list))
        .route("/:id", get(handle_get_one))
}

#[axum_macros::debug_handler]
async fn handle_list(State(state): State<AppState>, form: Form<ListContainerData>) -> Response {
    let payload: ListContainerData = form.0;
    let result = list_containers_usecase(&state.db_pool, payload).await;
    match result {
        Some(res) => {ServerResponse::ok(res).into_response()},
        None => {StatusCode::NOT_FOUND.into_response()}
    }
}

#[axum_macros::debug_handler]
async fn handle_get_one(State(state): State<AppState>,Path(user_id): Path<String>) -> Response {
    match  user_id.parse::<i32>() {
        Ok(id) => {
            let result = get_container_usecase(&state.db_pool, GetContainerData { user_id: id }).await;
            match result {
                Some(res) => {ServerResponse::ok(res).into_response()},
                None => {StatusCode::NOT_FOUND.into_response()}
            }
        }
        Err(err) => {
            StatusCode::BAD_REQUEST.into_response()
        }
    }
}