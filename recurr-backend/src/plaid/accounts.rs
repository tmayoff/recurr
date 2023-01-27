use recurr_core::{Account, Item};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use super::PlaidRequest;

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

#[tauri::command]
pub async fn accounts_get(
    auth_key: String,
    access_token: String,
    account_ids: Vec<String>,
) -> Result<Vec<Account>, String> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(&auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let options = if account_ids.is_empty() {
        None
    } else {
        Some(Options { account_ids })
    };

    let data = serde_json::to_value(AccountsGetRequest {
        access_token,
        options,
    })
    .expect("Failed to serialize");

    log::info!("{:?}", &data);

    let req = PlaidRequest {
        endpoint: "/accounts/get".to_string(),
        data,
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://linaejyblplchxcrusjy.functions.supabase.co/plaid".to_string())
        .json(&req)
        .headers(headers)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                log::info!("{:?}", res);
                let json: Vec<Account> = res.json().await.expect("Failed to deserialize data");
                Ok(json)
            } else {
                Err(res.text().await.expect("Failed to get text"))
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Serialize, Deserialize)]
struct AccountsBalanceGetRequest {
    account_ids: Vec<String>,
    min_last_updated_datetime: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountsBalanceGetResponse {
    accounts: Vec<Account>,
    item: Item,
}

#[tauri::command]
pub async fn balance_get(
    auth_key: String,
    account_ids: Vec<String>,
    last_updated: String,
) -> Result<AccountsBalanceGetResponse, String> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(&auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let req = AccountsBalanceGetRequest {
        account_ids,
        min_last_updated_datetime: last_updated,
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://linaejyblplchxcrusjy.functions.supabase.co/public_key_exchange".to_string())
        .json(&req)
        .headers(headers)
        .send()
        .await;

    match res {
        Ok(res) => {
            log::info!("{:?}", res);
            let json: AccountsBalanceGetResponse = res.json().await.unwrap();
            return Ok(json);
        }
        Err(err) => return Err(err.to_string()),
    }
}
