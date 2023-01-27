use serde::{Deserialize, Serialize};

pub mod access_token;
pub mod accounts;

#[derive(Debug, Serialize, Deserialize)]
pub enum SupabaseErrors {
    RequestError(String),
    SchemaError(String),
    QueryError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaAccessToken {
    #[serde(skip_serializing)]
    id: i32,
    access_token: String,
    user_id: String,

    plaid_accounts: Option<Vec<SchemaPlaidAccount>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SchemaPlaidAccount {
    user_id: String,
    account_id: String,
    access_token_id: i32,
}
