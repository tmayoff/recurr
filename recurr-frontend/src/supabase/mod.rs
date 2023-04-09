use serde::Deserialize;

pub mod transactions;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Session {
    #[serde(rename = "access_token")]
    pub auth_key: String,
    pub token_type: String,
    pub expires_in: f64,
    pub user: User,
}
