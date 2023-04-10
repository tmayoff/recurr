use recurr_core::{Account, Item};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use super::{Error, PlaidRequest};

#[derive(Serialize, Deserialize)]
struct Options {
    account_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct AccountsGetRequest {
    access_token: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Options>,
}

pub async fn get_balances(
    auth_key: &str,
    access_token: String,
    account_ids: Vec<String>,
) -> Result<Vec<Account>, Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let options = if account_ids.is_empty() {
        None
    } else {
        Some(Options { account_ids })
    };

    let data = serde_json::to_value(AccountsGetRequest {
        access_token: access_token.to_string(),
        options,
    })
    .expect("Failed to serialize");

    let req = PlaidRequest {
        endpoint: "/accounts/balance/get".to_string(),
        data: Some(data),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(env!("PLAID_URL"))
        .json(&req)
        .headers(headers)
        .send()
        .await
        .and_then(|e| e.error_for_status())
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    #[derive(Deserialize)]
    struct AccountsGetResponse {
        accounts: Vec<Account>,
        // item: Item,
    }

    let account_response = res
        .json::<AccountsGetResponse>()
        .await
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;

    Ok(account_response.accounts)
}

#[tauri::command]
pub async fn get_accounts(
    auth_key: &str,
    access_token: &str,
    account_ids: Vec<String>,
) -> Result<(Item, Vec<Account>), super::Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let options = if account_ids.is_empty() {
        None
    } else {
        Some(Options { account_ids })
    };

    let data = serde_json::to_value(AccountsGetRequest {
        access_token: access_token.to_string(),
        options,
    })
    .expect("Failed to serialize");

    let req = PlaidRequest {
        endpoint: "/accounts/get".to_string(),
        data: Some(data),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(env!("PLAID_URL"))
        .json(&req)
        .headers(headers)
        .send()
        .await
        .and_then(|e| e.error_for_status())
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    #[derive(Deserialize)]
    struct AccountsGetResponse {
        accounts: Vec<Account>,
        item: Item,
    }

    if res.status().is_success() {
        let account_response = res
            .json::<AccountsGetResponse>()
            .await
            .map_err(|e| recurr_core::Error::Request(e.to_string()))?;
        Ok((account_response.item, account_response.accounts))
    } else {
        let res = res
            .json()
            .await
            .map_err(|e| recurr_core::Error::Request(e.to_string()))?;
        Err(super::Error::Plaid(res))
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccountsBalanceGetResponse {
    accounts: Vec<Account>,
    item: Item,
}
