use sqlx;
use sqlx::PgPool;
use sqlx::Result;
use chrono::Utc;

use crate::model::user::User;
use crate::model::user_management_payload::UserManagementPayload;

pub struct DataUser {}

impl DataUser {
    pub async fn get_by_id(pool: &PgPool, id: i32) -> Option<User> {
        let result = sqlx::query_as!(User,
        "SELECT id, username, email FROM core.user WHERE id = $1",
        id)
            .fetch_one(pool)
            .await;
        match result {
            Ok(value)=> Some(value),
            Err(e) => {
                println!("{}",e);
                None
            }
        }
    }

    pub async fn auth_with_mail(pool: &PgPool, email: String, password: String) -> Option<User> {
        let result = sqlx::query_as!(User,
        "SELECT id, username, email FROM core.user WHERE email = $1 AND password= $2",
        email, password)
            .fetch_one(pool)
            .await;
        match result {
            Ok(value)=> Some(value),
            Err(e) => {
                println!("{}",e);
                None
            }
        }
    }

    pub async fn update_password(pool: &PgPool, id: i32, new_password: String) -> Option<User> {
        let result = sqlx::query_as!(User,
        "UPDATE core.user SET password = $2, token = NULL, token_created_at = NULL  WHERE id = $1 RETURNING id, username, email",
        id, new_password)
            .fetch_one(pool)
            .await;
        match result {
            Ok(value)=> Some(value),
            Err(e) => {
                println!("{}",e);
                None
            }
        }
    }

    pub async fn retrieve_management_data(pool: &PgPool, email: String) -> Option<UserManagementPayload> {
        let result = sqlx::query_as!(UserManagementPayload,
        "SELECT id, token, token_created_at FROM core.user WHERE email = $1",
        email)
            .fetch_one(pool)
            .await;
        match result {
            Ok(value)=> Some(value),
            Err(e) => {
                println!("{}",e);
                None
            }
        }
    }

    pub async fn set_reset_token(pool: &PgPool, email: &String, token: &String) -> Option<User> {
        let result = sqlx::query_as!(User,
        "UPDATE core.user SET token = $2, token_created_at = $3 WHERE email = $1 RETURNING id, username, email ",
        email, token, Utc::now())
            .fetch_one(pool)
            .await;
        match result {
            Ok(value)=> Some(value),
            Err(e) => {
                println!("{}",e);
                None
            }
        }
    }
}