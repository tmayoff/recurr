#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

const URL_ENDPOINT: &str = "https://sandbox.plaid.com";

#[derive(Debug, Deserialize)]
struct LinkCreate {
    expiration: String,
    link_token: String,
    request_id: String,
}

#[tauri::command]
async fn link_create() -> Result<String, String> {
    // link token

    let mut headers = HeaderMap::new();
    headers.insert(
        "PLAID-CLIENT-ID",
        HeaderValue::from_static("624e3a683b17e100151c96be"),
    );
    headers.insert(
        "PLAID-SECRET",
        HeaderValue::from_static("fd0ad9a62faaf6f7c07765ab72cf16"),
    );
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let req_body = r#"{
        "client_name": "Tyler Mayoff",
        "country_codes": ["CA"],
        "language": "en",
        "user": {
            "client_user_id": "tmayoff"
        },
        "products": ["auth"]
    }"#;

    let client = reqwest::Client::new();
    log::info!("Sending request {:?}", req_body);
    log::info!("Sending request {:?}", headers);
    let res = client
        .post(format!("{}/link/token/create", URL_ENDPOINT))
        .headers(headers)
        .body(req_body)
        .send()
        .await;

    match res {
        Ok(res) => {
            assert!(res.status().is_success());

            let json: LinkCreate = res.json().await.unwrap();
            log::info!("{:?}", json);

            return Ok(json.link_token);
        }
        Err(err) => {
            log::error!("{:?}", err);
        }
    }

    Err(String::from("Error"))
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![link_create])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
