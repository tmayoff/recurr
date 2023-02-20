use std::fmt::Display;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

pub mod accounts;
pub mod institutions;
pub mod link;
pub mod transactions;

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
enum PlaidErrorType {
    INVALID_REQUEST,
    INVALID_RESULT,
    INVALID_INPUT,
    INSTITUTION_ERROR,
    RATE_LIMIT_EXCEEDED,
    API_ERROR,
    ITEM_ERROR,
    ASSET_REPORT_ERROR,
    RECAPTCHA_ERROR,
    OAUTH_ERROR,
    PAYMENT_ERROR,
    BANK_TRANSFER_ERROR,
    INCOME_VERIFICATION_ERROR,
    MICRODEPOSITS_ERROR,
}

impl Display for PlaidErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, thiserror::Error)]
pub struct PlaidError {
    error_type: PlaidErrorType,
    error_code: String,
    error_message: String,
    display_message: Option<String>,
}

impl Display for PlaidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Plaid returned an error (type: {}), (code: {}), (error message: {}), (display message: {:?})",
            self.error_type, self.error_code, self.error_message, self.display_message
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    EnVar(#[from] std::env::VarError),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),

    #[error(transparent)]
    Plaid(#[from] PlaidError),

    #[error("{0}")]
    Other(String),
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
    data: Option<serde_json::Value>,
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

pub async fn item_remove(auth_key: &str, access_token: &str) -> Result<(), Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    #[derive(Serialize)]
    struct Request {
        access_token: String,
    }

    let data = serde_json::to_value(Request {
        access_token: access_token.to_string(),
    })?;

    let req = PlaidRequest {
        endpoint: "/item/remove".to_string(),
        data: Some(data),
    };

    let client = reqwest::Client::new();
    let _ = client
        .post(env!("PLAID_URL"))
        .json(&req)
        .headers(headers)
        .send()
        .await?;

    Ok(())
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
        data: Some(data),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(env!("PLAID_URL"))
        .json(&req)
        .headers(headers)
        .send()
        .await?;

    let json = res.error_for_status()?.json().await?;
    Ok(json)
}
