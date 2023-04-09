use recurr_core::{plaid::link::LinkToken, Institution};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn invokeLinkTokenCreate(
        anon_key: &str,
        user_id: &str,
        access_token: Option<String>,
    ) -> Result<JsValue, JsValue>;

    fn linkStart(link_token: &str, callback: JsValue);
}

pub async fn link_token_create(
    anon_key: &str,
    user_id: &str,
    access_token: Option<String>,
) -> Result<LinkToken, String> {
    let response = invokeLinkTokenCreate(anon_key, user_id, access_token).await;

    match response {
        Ok(response) => {
            let res =
                serde_wasm_bindgen::from_value::<LinkToken>(response).expect("Response not valid");
            Ok(res)
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub mask: Option<String>,

    #[serde(rename = "type")]
    pub account_type: String,

    pub subtype: String,

    pub verification_status: Option<String>,
    pub class_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub institution: Option<Institution>,
    pub accounts: Vec<Account>,
    pub link_session_id: String,
    pub transfer_status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LinkSuccess {
    pub public_token: String,
    pub metadata: Metadata,
}
#[derive(Debug, Deserialize)]
pub struct LinkFailure {
    err: String,
}

pub fn start(
    link_token: String,
    mut callback: impl FnMut(Result<LinkSuccess, LinkFailure>) + 'static,
) {
    linkStart(
        &link_token,
        Closure::once_into_js(move |response: JsValue| {
            let s = serde_wasm_bindgen::from_value::<LinkSuccess>(response.clone());
            if let Ok(success) = s {
                callback(Ok(success));
                return;
            };

            let e = serde_wasm_bindgen::from_value::<LinkFailure>(response);
            if let Ok(failure) = e {
                log::error!("Failure: {:?}", failure);
                return;
            };

            log::error!("Success deserialized failure: {:?}", s);
            log::error!("Failure deserialized failure: {:?}", e);
        }),
    );
}
