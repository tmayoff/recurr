use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct User {
    pub client_user_id: String,
}

#[derive(Serialize, Debug)]
pub struct LinkTokenCreateRequest {
    pub client_name: String,
    pub language: String,
    pub country_codes: Vec<String>,
    pub products: Vec<String>,
    pub user: User,
}

impl LinkTokenCreateRequest {
    pub fn new(
        client_name: &str,
        language: &str,
        country_codes: Vec<String>,
        products: Vec<String>,
        user: User,
    ) -> Self {
        Self {
            client_name: client_name.to_string(),
            language: language.to_string(),
            country_codes: country_codes.to_vec(),
            products: products,
            user: user,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkTokenCreateReponse {
    expiration: String,
    link_token: String,
    request_id: String,
}

#[tauri::command]
pub async fn link_token_create(anon_key: &str) -> Result<LinkTokenCreateReponse, String> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(&anon_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let req = LinkTokenCreateRequest::new(
        "Recurr",
        "en",
        vec!["CA".to_owned()],
        vec!["auth".to_owned()],
        User {
            client_user_id: "tmayoff".to_owned(),
        },
    );

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "https://linaejyblplchxcrusjy.functions.supabase.co/link_create"
        ))
        .json(&req)
        .headers(headers)
        .send()
        .await;

    match res {
        Ok(res) => {
            let json: LinkTokenCreateReponse = res.json().await.unwrap();
            return Ok(json);
        }
        Err(err) => {
            log::error!("{:?}", err);
        }
    }

    Err(String::from("Error"))
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
    anon_key: &str,
    public_token: &str,
) -> Result<PublicTokenExchangeResponse, String> {
    let mut authorization = String::from("Bearer ");
    authorization.push_str(&anon_key);

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
