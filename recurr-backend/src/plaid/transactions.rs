use recurr_core::{Category, Transaction, TransactionOption, Transactions};
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
        .await
        .map(|e| e.error_for_status())
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    #[derive(Deserialize)]
    struct Response {
        categories: Vec<Category>,
    }

    let json: Response = res
        .json()
        .await
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;
    Ok(json.categories)
}

#[tauri::command]
pub async fn sync(auth_key: &str, access_token: &str) -> Result<(), Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    // TODO call plaid /sync endpoint repeatedly
    #[derive(Serialize)]
    struct Request {
        access_token: String,
        cursor: Option<String>,
    }

    let data = serde_json::to_value(Request {
        access_token: access_token.to_string(),
        cursor: None,
    })?;

    let req = PlaidRequest {
        endpoint: "/transactions/sync".to_string(),
        data: Some(data),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(env!("PLAID_URL"))
        .json(&req)
        .headers(headers)
        .send()
        .await
        .map(|e| e.error_for_status())
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    #[derive(Debug, Deserialize)]
    struct Response {
        added: Vec<Transaction>,
        modified: Vec<Transaction>,
        removed: Vec<String>,
        next_cursor: String,
        has_more: bool,
        request_id: String,
    }

    let res = res.json::<Response>().await;
    log::info!("{:?}", res);

    // TODO save response in Database

    Ok(())
}

#[tauri::command]
pub async fn get_transactions(
    _auth_key: &str,
    _access_token: &str,
    _start_date: String,
    _end_date: String,
    _options: TransactionOption,
) -> Result<Transactions, Error> {
    // let mut authorization = String::from("Bearer ");
    // authorization.push_str(auth_key);

    // let mut headers = HeaderMap::new();
    // headers.insert("Authorization", authorization.parse().unwrap());
    // headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    // #[derive(Serialize)]
    // struct Request {
    //     access_token: String,
    //     options: TransactionOption,

    //     start_date: String,
    //     end_date: String,
    // }

    // let data = serde_json::to_value(Request {
    //     access_token: access_token.to_string(),
    //     options,
    //     start_date,
    //     end_date,
    // })?;

    // let req = PlaidRequest {
    //     endpoint: "/transactions/get".to_string(),
    //     data: Some(data),
    // };

    // let client = reqwest::Client::new();
    // let res = client
    //     .post(env!("PLAID_URL"))
    //     .json(&req)
    //     .headers(headers)
    //     .send()
    //     .await?
    //     .error_for_status()?;

    // Ok(res.json().await?)
    todo!("Replace with transactions/sync")
}
