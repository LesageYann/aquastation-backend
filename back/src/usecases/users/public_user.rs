use serde::{Serialize};

use api_types::user::User;

#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub username: String,
    pub id: i32
}

impl PublicUser {
    pub fn from_user(user: User) -> PublicUser {
        PublicUser{username: user.username, id: user.id}
    }
}