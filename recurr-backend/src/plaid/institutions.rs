use recurr_core::Institution;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::plaid::PlaidRequest;

#[tauri::command]
pub async fn get_institution(
    auth_key: &str,
    institution_id: &str,
) -> Result<Institution, super::Error> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(auth_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    #[derive(Serialize)]
    struct InstitutionGetRequest {
        institution_id: String,
        country_codes: Vec<String>,
    }

    let data = serde_json::to_value(InstitutionGetRequest {
        institution_id: institution_id.to_string(),
        country_codes: vec!["CA".to_string()],
    })
    .expect("Failed to serialize");

    let req = PlaidRequest {
        endpoint: "/institutions/get_by_id".to_string(),
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

    #[derive(Deserialize)]
    struct Response {
        institution: Institution,
    }

    let response: Response = res
        .json()
        .await
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;

    Ok(response.institution)
}
