use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::data_sources::users::DataUser;
use crate::model::error::ASError;
use crate::usecases::users::auth_body::AuthBody;
use crate::usecases::users::claims::Claims;
use crate::usecases::users::public_user::PublicUser;

#[derive(Deserialize, Debug)]
pub struct LogInBody {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct LogInResult {
    auth: AuthBody,
    user: PublicUser
}

pub async fn log_in_usecase(pool: &PgPool, payload: LogInBody, secret: &[u8] ) -> Result<LogInResult, ASError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ASError::MissingCredentials);
    }

    let user = DataUser::auth_with_mail(pool, payload.email, payload.password).await;
    match user {
        Some(user) => {
            let claims = Claims::new(user.id);
            // Create the authorization token
            let token = claims.encode_token(secret)
                .map_err(|_| ASError::TokenCreation)?;

            // Send the authorized token
            Ok(LogInResult { auth: AuthBody::new(token), user: PublicUser::from_user(user) })
        }
        None => {Err(ASError::WrongCredentials)}
    }
}
