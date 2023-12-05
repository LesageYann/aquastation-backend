use axum::{Form, Router};
use axum::body::Body;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};

use crate::api::app_state::{AppState, AppStateTrait};
use crate::api::helpers::server_response::ServerResponse;
use crate::usecases::users::log_in::{log_in_usecase, LogInBody};
use crate::usecases::users::request_reset_password::{request_reset_password_usecase, RequestResetBody};
use crate::usecases::users::reset_password::{reset_password_usecase, ResetPasswordPayload};
use crate::usecases::users::sign_in::{SignInBody, sign_in_usecase};

pub fn build_users_router() -> Router<AppState, Body>  {
    Router::new()
        .route("/SignIn", post(handle_sign_in))
        .route("/LogIn", post(handle_log_in))
        .route("/password/reset", post(handle_reset_password))
        .route("/password/request", post(handle_request_reset))
}

async fn handle_sign_in(State(state): State<AppState>, form: Form<SignInBody> ) -> Response {
    let payload = form.0;
    let result = sign_in_usecase(&state.db_pool, payload, state.get_secret()).await;
    match result {
        Ok(token) => {ServerResponse::ok(token).into_response()},
        Err(e) => {e.into_response()}
    }
}

async fn handle_log_in(State(state): State<AppState>, form: Form<LogInBody> ) -> Response {
    let payload = form.0;
    let result = log_in_usecase(&state.db_pool, payload, state.get_secret()).await;
    match result {
        Ok(token) => {ServerResponse::ok(token).into_response()},
        Err(e) => {e.into_response()}
    }
}

async fn handle_reset_password(State(state): State<AppState>, form: Form<ResetPasswordPayload> ) -> Response {
    let payload: ResetPasswordPayload = form.0;
    let result = reset_password_usecase(&state.db_pool, payload, state.get_secret(), &state.token_duration).await;
    match result {
        Ok(token) => {ServerResponse::ok(token).into_response()},
        Err(e) => {e.into_response()}
    }
}

async fn handle_request_reset(State(state): State<AppState>, form: Form<RequestResetBody> ) -> Response {
    let payload: RequestResetBody = form.0;
    let result = request_reset_password_usecase(&state.db_pool, payload, &state.domain, &state.smtp_credential, &state.template_engine).await;
    match result {
        Ok(token) => {ServerResponse::ok(token).into_response()},
        Err(e) => {e.into_response()}
    }
}