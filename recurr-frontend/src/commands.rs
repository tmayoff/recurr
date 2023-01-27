use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn invokeLinkTokenCreate(anon_key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeItemPublicTokenExchange(
        anon_key: &str,
        public_token: &str,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeSaveAccessToken(
        auth_token: &str,
        user_id: &str,
        access_token: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeSavePlaidAccount(
        auth_token: &str,
        user_id: &str,
        access_token: &str,
        plaid_account_id: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeGetPlaidAccounts(
        auth_token: &str,
        user_id: &str,
    ) -> Result<JsValue, JsValue>;

    pub fn linkStart(link_token: String, callback: JsValue);
}

pub async fn get_all_accounts(auth_token: &str, user_id: &str) -> Vec<recurr_core::Account> {
    let account_ids = invokeGetPlaidAccounts(auth_token, user_id).await;

    log::info!("{:?}", account_ids);

    Vec::new()
}
