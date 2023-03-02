use axum::{Form, Router};
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};

use crate::api::app_state::AppState;
use crate::api::helpers::server_response::{ServerResponse};
use crate::usecases::probes::get::{GetProbeData, get_probe_usecase};
use crate::usecases::probes::list::{ListProbesData, list_probes_usecase};

pub fn build_probes_router() -> Router<AppState, Body> {
    Router::new()
        .route("/SignIn", post(handle_list))
        .route("/:id", get(handle_get_one))
}

#[axum_macros::debug_handler]
async fn handle_list(State(state): State<AppState>, form: Form<ListProbesData>) -> Response {
    let payload: ListProbesData = form.0;
    let result = list_probes_usecase(&state.db_pool, payload).await;
    match result {
        Some(res) => {ServerResponse::ok(res).into_response()},
        None => {StatusCode::NOT_FOUND.into_response()}
    }
}

#[axum_macros::debug_handler]
async fn handle_get_one(State(state): State<AppState>, Path(probe_id): Path<String>) -> Response {
    match  probe_id.parse::<i32>() {
        Ok(id) => {
            let result = get_probe_usecase(&state.db_pool, GetProbeData { probe_id: id }).await;
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