use recurr_core::{Account, Category, Transaction, TransactionOption};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::plaid::PlaidRequest;

use super::Error;

#[tauri::command]
pub async fn get_categories() -> Result<Vec<Category>, Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(env!("SUPABASE_KEY"));

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let req = PlaidRequest {
        endpoint: "/categories/get".to_string(),
        data: None,
    };

    let client = reqwest::Client::new();
    let res = client
        .post(env!("PLAID_URL"))
        .json(&req)
        .headers(headers)
        .send()
        .await?
        .error_for_status()?;

    #[derive(Deserialize)]
    struct Response {
        categories: Vec<Category>,
    }

    let json: Response = res.json().await?;
    Ok(json.categories)
}

#[tauri::command]
pub async fn get_transactions(
    auth_key: &str,
    access_token: &str,
    start_date: String,
    end_date: String,
    options: TransactionOption,
) -> Result<(Vec<Account>, Vec<Transaction>), Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    #[derive(Serialize)]
    struct Request {
        access_token: String,
        options: TransactionOption,

        start_date: String,
        end_date: String,
    }

    let data = serde_json::to_value(Request {
        access_token: access_token.to_string(),
        options,
        start_date,
        end_date,
    })?;

    let req = PlaidRequest {
        endpoint: "/transactions/get".to_string(),
        data: Some(data),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(env!("PLAID_URL"))
        .json(&req)
        .headers(headers)
        .send()
        .await?
        .error_for_status()?;

    #[derive(Debug, Deserialize)]
    struct Response {
        accounts: Vec<Account>,
        transactions: Vec<Transaction>,
    }

    let json: Response = res.json().await?;
    Ok((json.accounts, json.transactions))
}
