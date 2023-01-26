use recurr_core::{Account, Item};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

pub mod link;

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
) -> Result<PublicTokenExchangeResponse, String> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(&auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let req = PublicTokenExchangeRequest {
        public_token: public_token.to_owned(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "https://linaejyblplchxcrusjy.functions.supabase.co/public_key_exchange"
        ))
        .json(&req)
        .headers(headers)
        .send()
        .await;

    match res {
        Ok(res) => {
            log::info!("{:?}", res);
            let json: PublicTokenExchangeResponse = res.json().await.unwrap();
            return Ok(json);
        }
        Err(err) => return Err(err.to_string()),
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
pub async fn accounts_balance_get(
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
        .post(format!(
            "https://linaejyblplchxcrusjy.functions.supabase.co/public_key_exchange"
        ))
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
