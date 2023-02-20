use std::fmt::Display;

use postgrest::Postgrest;

use crate::plaid;

pub mod access_token;
pub mod accounts;

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
    let client = Postgrest::new(env!("SUPABASE_URL").to_owned() + "/rest/v1")
        .insert_header("apikey", env!("SUPABASE_KEY"));

    Ok(client)
}
