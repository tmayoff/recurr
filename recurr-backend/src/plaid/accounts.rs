use recurr_core::{Account, Institution, Item};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::{plaid::institutions::institution_get, supabase::accounts::get_plaid_accounts};

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
    auth_key: &str,
    access_token: &str,
    account_ids: Vec<String>,
) -> Result<(Institution, Vec<Account>), super::Error> {
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
        data,
    };

    let client = reqwest::Client::new();
    let res = client
        .post(std::env::var("PLAID_URL")?)
        .json(&req)
        .headers(headers)
        .send()
        .await?;

    #[derive(Deserialize)]
    struct AccountsGetResponse {
        accounts: Vec<Account>,
        item: Item,
    }

    let account_response = res
        .error_for_status()?
        .json::<AccountsGetResponse>()
        .await?;

    let id = account_response
        .item
        .institution_id
        .expect("No institution associated with this");
    let institution = institution_get(auth_key, &id).await?;

    Ok((institution, account_response.accounts))
}

#[derive(Serialize, Deserialize)]
pub struct AccountsBalanceGetResponse {
    accounts: Vec<Account>,
    item: Item,
}

#[tauri::command]
pub async fn get_balances(
    auth_token: &str,
    access_token: &str,
    account_ids: Vec<String>,
) -> Result<Vec<Account>, super::Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(auth_token);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    #[derive(Serialize)]
    struct Options {
        account_ids: Vec<String>,
    }

    #[derive(Serialize)]
    struct Request {
        access_token: String,
        options: Options,
    }

    let data = serde_json::to_value(Request {
        access_token: access_token.to_string(),
        options: Options { account_ids },
    })
    .expect("Failed to serialize");

    let req = PlaidRequest {
        endpoint: "/accounts/balance/get".to_string(),
        data,
    };

    let client = reqwest::Client::new();
    let res = client
        .post(std::env::var("PLAID_URL")?)
        .json(&req)
        .headers(headers)
        .send()
        .await?;

    #[derive(Deserialize)]
    struct AccountsGetResponse {
        accounts: Vec<Account>,
        item: Item,
    }

    let account_response = res
        .error_for_status()?
        .json::<AccountsGetResponse>()
        .await?;

    log::info!("{:?}", &account_response.accounts);
    Ok(account_response.accounts)
}
