use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::data_sources::users::DataUser;
use crate::model::error::ASError;
use api_types::user_management_payload::UserManagementPayload;
use crate::usecases::users::auth_body::AuthBody;
use crate::usecases::users::claims::Claims;

#[derive(Deserialize, Debug)]
pub struct ResetPasswordPayload {
    token: String,
    email: String,
    new_password: String,
}

#[derive(Serialize, Debug)]
pub struct ResetPasswordResponse {
    token: String,
}

pub async fn reset_password_usecase(pool: &PgPool, payload: ResetPasswordPayload, secret: &[u8], token_duration: &i64) -> Result<AuthBody, ASError> {
    let find = DataUser::retrieve_management_data(pool, payload.email).await;

    match find {
        Some(data) => {
            if has_valid_token(&data, &payload.token, token_duration) {
                match DataUser::update_password(pool, data.id, payload.new_password).await {
                    Some(user) => {
                        let claims = Claims::new(user.id);
                        // Create the authorization token
                        let token = claims.encode_token(secret)
                            .map_err(|_| ASError::TokenCreation)?;

                        // Send the authorized token
                        Ok(AuthBody::new(token))
                    },
                    None => {
                        Err(ASError::InternalError)
                    }
                }
            } else {
                Err(ASError::InvalidRequest(vec!["Invalid token or emails".to_string()]))
            }
        }
        None => { Err(ASError::InvalidRequest(vec!["Invalid token or emails".to_string()])) }
    }
}

fn has_valid_token(data: &UserManagementPayload, token_received: &String, token_duration: &i64) -> bool {
    data.token.as_ref().map(|token| token == token_received).unwrap_or(false) && data.token_created_at.as_ref().map(|date| Utc::now().signed_duration_since(*date).num_seconds() <= *token_duration).unwrap_or(false)
}
