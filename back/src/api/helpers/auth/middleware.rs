use axum::{async_trait, extract::{FromRequestParts, TypedHeader}, RequestPartsExt};
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::http::{request::Parts};

use crate::api::app_state::AppStateTrait;
use crate::model::error::ASError;
use crate::usecases::users::claims::Claims;

#[async_trait]
impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync + AppStateTrait,
{
    type Rejection = ASError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ASError::InvalidToken)?;
        // Decode the user data
        let claims = Claims::extract_token(bearer.token().to_string(), state.get_secret())
            .map_err(|_| ASError::InvalidToken)?;

        Ok(claims)
    }
}