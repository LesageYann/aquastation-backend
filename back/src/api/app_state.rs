use sqlx::{PgPool};
use tera::Tera;
use crate::templates::get_engine;

#[derive(Clone)]
pub struct ServiceCredential {
    pub password: String,
    pub user: String,
    pub host: String,
}

pub trait AppStateTrait {
    fn get_secret(&self) -> &[u8];
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub auth_secret: String,
    pub domain: String,
    pub token_duration: i64,
    pub smtp_credential: ServiceCredential,
    pub template_engine: Tera,
}

impl AppState {
    pub async fn new() -> AppState {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let auth_secret = std::env::var("AUTH_SECRET").expect("AUTH_SECRET must be set.");
        let domain = std::env::var("URL").expect("URL must be set.");
        let smtp_user = std::env::var("SMTP_USER").expect("SMTP_USER must be set.");
        let smtp_password = std::env::var("SMTP_SECRET").expect("SMTP_SECRET must be set.");
        let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set.");

        let token_duration = std::env::var("TOKEN_DURATION").unwrap_or("3600".to_string()).parse::<i64>().expect("TOKEN_DURATION should be an integer"); // one hour by default

        match PgPool::connect(&db_url).await {
            Ok(db_pool) => {
                AppState {
                    db_pool,
                    auth_secret,
                    token_duration,
                    domain,
                    smtp_credential: ServiceCredential { user: smtp_user, password: smtp_password, host: smtp_host },
                    template_engine: get_engine()
                }
            }
            Err(e) => panic!("pg pool error : {}", e)
        }
    }
}

impl AppStateTrait for AppState {
    fn get_secret(&self) -> &[u8] {
        self.auth_secret.as_bytes()
    }
}