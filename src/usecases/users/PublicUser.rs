use serde::{Serialize};

use crate::model::user::User;

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