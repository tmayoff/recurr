use recurr_core::Institution;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::plaid::PlaidRequest;

pub async fn institution_get(
    auth_key: &str,
    insitution_id: &str,
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
        institution_id: insitution_id.to_string(),
        country_codes: vec!["CA".to_string()],
    })
    .expect("Failed to serialize");

    let req = PlaidRequest {
        endpoint: "/institutions/get_by_id".to_string(),
        data,
    };

    let client = reqwest::Client::new();
    let res = client
        .post(env!("PLAID_URL"))
        .json(&req)
        .headers(headers)
        .send()
        .await?;

    #[derive(Deserialize)]
    struct Response {
        institution: Institution,
    }

    let response: Response = res.error_for_status()?.json().await?;

    Ok(response.institution)
}
