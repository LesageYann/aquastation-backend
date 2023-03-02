use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::data_sources::users::DataUser;
use crate::model::error::ASError;
use crate::usecases::users::auth_body::AuthBody;
use crate::usecases::users::claims::Claims;

#[derive(Deserialize, Debug)]
pub struct SignInBody {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct SignInResult {
    token: String,
}

pub async fn sign_in_usecase(pool: &PgPool, payload: SignInBody, secret: &[u8] ) -> Result<AuthBody, ASError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ASError::MissingCredentials);
    }

    let user = DataUser::auth_with_mail(pool, payload.email, payload.password).await;
    match user {
        Some(user) => {
            let claims = Claims::new(user.id.to_string());
            // Create the authorization token
            let token = claims.encode_token(secret)
                .map_err(|_| ASError::TokenCreation)?;

            // Send the authorized token
            Ok(AuthBody::new(token))
        }
        None => {Err(ASError::WrongCredentials)}
    }
}
