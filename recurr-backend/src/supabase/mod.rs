use std::fmt::Display;

use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::plaid;

pub mod access_token;
pub mod accounts;
pub mod auth;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),

    #[error(transparent)]
    Plaid(#[from] plaid::Error),

    Query(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_supbase_client() -> Result<Postgrest, Error> {
    let client = Postgrest::new(env!("SUPABASE_URL")).insert_header("apikey", env!("SUPABASE_KEY"));

    Ok(client)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaAccessToken {
    #[serde(skip_serializing)]
    id: i32,
    access_token: String,
    user_id: String,

    #[serde(skip_serializing)]
    plaid_accounts: Option<Vec<SchemaPlaidAccount>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SchemaPlaidAccount {
    user_id: String,
    account_id: String,
    access_token_id: i32,
}
