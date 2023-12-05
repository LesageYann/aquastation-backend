use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct User {
    pub username: String,
    pub id: i32,
    pub token: String,
}

impl User {
    pub fn default() -> User {
        User { username: "".to_string(), id: 0, token: "".to_string() }
    }

    pub fn new(username: String, id: i32, token: String) -> User {
        User { username, id, token }
    }

    pub fn is_default(&self) -> bool {
        self.username.is_empty()
    }
}