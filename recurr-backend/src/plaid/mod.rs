use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

pub mod accounts;
pub mod institutions;
pub mod link;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Serialize)]
pub struct PlaidRequest {
    endpoint: String,
    data: serde_json::Value,
}

#[derive(Serialize, Debug)]
pub struct User {
    pub client_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PublicTokenExchangeRequest {
    pub public_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicTokenExchangeResponse {
    pub access_token: String,
    pub item_id: String,
    pub request_id: String,
}

#[tauri::command]
pub async fn item_public_token_exchange(
    auth_key: &str,
    public_token: &str,
) -> Result<PublicTokenExchangeResponse, Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let data = serde_json::to_value(PublicTokenExchangeRequest {
        public_token: public_token.to_owned(),
    })?;

    let req = PlaidRequest {
        endpoint: "/item/public_token/exchange".to_string(),
        data,
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://linaejyblplchxcrusjy.functions.supabase.co/plaid".to_string())
        .json(&req)
        .headers(headers)
        .send()
        .await?;

    let json = res.json().await?;
    Ok(json)
}
