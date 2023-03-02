use chrono::Utc;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use jsonwebtoken::errors::Error;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(user: String) -> Claims {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::days(1))
            .expect("valid timestamp")
            .timestamp();

        Claims { sub: user, exp: usize::try_from(expiration).unwrap() }
    }

    pub fn extract_token(token: String, secret: &[u8]) -> Result<Claims, Error> {
        decode::<Claims>(&token, &DecodingKey::from_secret(secret), &Validation::default())
            .map(|token | token.claims)
    }

    pub fn encode_token(&self, secret: &[u8]) -> Result<String, Error> {
        encode(&Header::default(), self, &EncodingKey::from_secret(secret))
    }
}