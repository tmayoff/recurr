use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Session {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub user: User,
}
