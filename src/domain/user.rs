use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub struct Users(pub Vec<User>);
