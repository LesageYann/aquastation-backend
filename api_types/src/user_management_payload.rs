use chrono::{DateTime, Utc};
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct UserManagementPayload {
    pub id: i32,
    pub token: Option<String>,
    pub token_created_at: Option<DateTime<Utc>>,
}

impl UserManagementPayload {
    pub fn new(id: i32, token: Option<String>, token_created_at: Option<DateTime<Utc>>,) -> UserManagementPayload {
        UserManagementPayload { id, token, token_created_at}
    }
}