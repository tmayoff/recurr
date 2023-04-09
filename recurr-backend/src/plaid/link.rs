use recurr_core::plaid::link::LinkToken;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;

use super::{PlaidRequest, User};

#[tauri::command]
pub async fn link_token_create(
    auth_key: &str,
    user_id: &str,
    access_token: Option<String>,
) -> Result<LinkToken, super::Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    #[derive(Serialize, Debug)]
    struct Request {
        pub client_name: String,
        pub language: String,
        pub country_codes: Vec<String>,
        pub products: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_token: Option<String>,
        pub user: User,
    }

    let data = serde_json::to_value(Request {
        access_token,
        client_name: "Recurr".to_string(),
        language: "en".to_string(),
        country_codes: vec!["CA".to_string(), "US".to_string()],
        products: vec![
            "auth".to_string(),
            "transactions".to_string(),
            "investments".to_string(),
            "liabilities".to_string(),
        ],
        user: User {
            client_user_id: user_id.to_string(),
        },
    })?;

    let req = PlaidRequest {
        endpoint: "/link/token/create".to_string(),
        data: Some(data),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(env!("PLAID_URL"))
        .json(&req)
        .headers(headers)
        .send()
        .await
        .map(|r| r.error_for_status())
        .flatten()
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;

    let json = res
        .json()
        .await
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;
    Ok(json)
}
