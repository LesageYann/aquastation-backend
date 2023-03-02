use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(id: i32, username: String, email: String,) -> User {
        User { id, username, email}
    }
}