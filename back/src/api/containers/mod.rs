use std::collections::HashMap;
use axum::{Form, Router};
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde::Deserialize;
use api_types::container::ContainerWithStatus;
use api_types::probes_status::ProbeState;

use crate::api::app_state::AppState;
use crate::api::helpers::server_response::ServerResponse;
use crate::model::error::ASError;
use crate::usecases::containers::create::{create_container_usecase, CreateBody};
use crate::usecases::containers::get::{GetContainerData, get_container_usecase};
use crate::usecases::containers::list::{ListContainerData, list_containers_usecase};
use crate::usecases::probes::get_states::{get_states, GetStatesData};
use crate::usecases::users::claims::Claims;

#[derive(Debug, Deserialize)]
struct GetParams {
    #[serde(default)]
    probes_states: Option<bool>,
}

pub fn build_containers_router() -> Router<AppState, Body> {
    Router::new()
        .route("/", get(handle_list).post(handle_create_one))
        .route("/:id", get(handle_get_one))
}

#[axum_macros::debug_handler]
async fn handle_list(State(state): State<AppState>, claim: Claims, query: Query<GetParams>) -> Response {
    println!("handle list");
    let payload: ListContainerData = ListContainerData {
        user_id: claim.sub,
        limit: None,
        offset: None,
        container_type: None,
    };
    let result = list_containers_usecase(&state.db_pool, payload).await;
    match result {
        Some(res) => {
            match query.probes_states {
                Some(true) => {
                    let ids = res.iter().map(|container| container.id).collect::<Vec<i32>>();
                    let states = get_states(
                        &state.db_pool,
                        GetStatesData { containers_id: ids, user_id: claim.sub },
                    ).await.unwrap_or(HashMap::new());
                    println!("res {}", res.len());
                    let with_states = res.into_iter().fold(Vec::new(), |mut acc, container| {
                        let status = match states.get(&container.id) {
                            None => { Vec::new() }
                            Some(value) => { value.to_vec() }
                        };
                        acc.push(ContainerWithStatus { status, props: container });
                        acc
                    });
                    ServerResponse::ok(with_states).into_response()
                }
                _ => {
                    println!("res {}", res.len());
                    ServerResponse::ok(res).into_response()
                }
            }
        }
        None => { StatusCode::NOT_FOUND.into_response() }
    }
}

#[axum_macros::debug_handler]
async fn handle_get_one(State(state): State<AppState>, claim: Claims, Path(container_id): Path<String>) -> Response {
    match container_id.parse::<i32>() {
        Ok(id) => {
            let result = get_container_usecase(&state.db_pool, GetContainerData { user_id: claim.sub, container_id: id }).await;
            match result {
                Some(res) => { ServerResponse::ok(res).into_response() }
                None => { StatusCode::NOT_FOUND.into_response() }
            }
        }
        Err(_err) => {
            let mut err = Vec::new();
            err.push("invalid id".to_string());
            ASError::InvalidRequest(err).into_response()
        }
    }
}

async fn handle_create_one(State(state): State<AppState>, claim: Claims, form: Form<CreateBody>) -> Response {
    let mut errors = Vec::new();
    let payload = form.0;

    if payload.name.is_empty() {
        errors.push("invalid name".to_string());
    }

    if payload.volume < 0 {
        errors.push("invalid volume".to_string());
    }

    if !errors.is_empty() {
        ASError::InvalidRequest(errors).into_response()
    } else {
        let result = create_container_usecase(&state.db_pool, claim, payload).await;
        match result {
            Ok(res) => { ServerResponse::ok(res).into_response() }
            Err(_) => { StatusCode::NOT_FOUND.into_response() }
        }
    }
}