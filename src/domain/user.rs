use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub struct Users(pub Vec<User>);
