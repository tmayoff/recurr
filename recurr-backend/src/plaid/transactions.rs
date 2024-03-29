use async_recursion::async_recursion;
use recurr_core::{Category, Transaction};
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
        .flatten()
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
#[async_recursion]
pub async fn sync(auth_key: &str, access_token: &str, cursor: Option<String>) -> Result<(), Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    #[derive(Serialize)]
    struct Request {
        access_token: String,
        cursor: Option<String>,
    }

    let data = serde_json::to_value(Request {
        access_token: access_token.to_string(),
        cursor,
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
        .flatten()
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    #[derive(Debug, Deserialize)]
    struct Response {
        added: Vec<Transaction>,
        modified: Vec<Transaction>,
        removed: Vec<String>,
        next_cursor: String,
        has_more: bool,
        //        request_id: String,
    }

    let plaid_response = res
        .json::<Response>()
        .await
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    let client = recurr_core::get_supbase_client();

    let _ = client
        .from("transactions")
        .auth(auth_key)
        .upsert(serde_json::to_string(&plaid_response.added).expect("Failed to serialize"))
        .execute()
        .await
        .map(|e| e.error_for_status())
        .flatten()
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    let _ = client
        .from("transactions")
        .auth(auth_key)
        .upsert(serde_json::to_string(&plaid_response.modified)?)
        .execute()
        .await
        .map(|e| e.error_for_status())
        .flatten()
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    let _ = client
        .from("transactions")
        .auth(auth_key)
        .delete()
        .in_("transaction_id", plaid_response.removed)
        .execute()
        .await
        .map(|e| e.error_for_status())
        .flatten()
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    if plaid_response.has_more {
        sync(auth_key, access_token, Some(plaid_response.next_cursor)).await?;
    }

    Ok(())
}
