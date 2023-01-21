use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct User {
    id: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Session {
    access_token: String,
    token_type: String,
    expires_in: u32,
    user: User,
}
