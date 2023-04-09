use recurr_core::{Account, Category, Institution, Item};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod link;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {

    #[wasm_bindgen(catch)]
    pub async fn invokeRemoveAccount(
        user_id: &str,
        auth_key: &str,
        access_token: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeItemPublicTokenExchange(
        anon_key: &str,
        public_token: &str,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeGetInstitution(
        auth_key: &str,
        id: Option<String>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeGetAccounts(
        auth_key: &str,
        access_token: &str,
        account_ids: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeGetCategories() -> Result<JsValue, JsValue>;

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
    pub async fn invokeGetPlaidBalances(
        auth_token: &str,
        user_id: &str,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeTransactionsSync(
        auth_token: &str,
        access_token: &str,
    ) -> Result<(), JsValue>;
}

pub async fn get_accounts(
    auth_key: &str,
    access_token: &str,
    account_ids: Vec<String>,
) -> Result<(Item, Vec<Account>), recurr_core::Error> {
    let account_ids = serde_wasm_bindgen::to_value(&account_ids).expect("failed to serialize");

    let res = invokeGetAccounts(auth_key, access_token, account_ids).await;
    match res {
        Ok(accounts) => {
            let accounts: (Item, Vec<Account>) =
                serde_wasm_bindgen::from_value(accounts).expect("Failed to deserialize data");
            Ok(accounts)
        }
        Err(e) => {
            Err(serde_wasm_bindgen::from_value::<recurr_core::Error>(e)
                .expect("Failed to deserialize"))
        }
    }
}

pub async fn get_categories() -> Result<Vec<Category>, String> {
    let res = invokeGetCategories()
        .await
        .map_err(|e| e.as_string().unwrap())?;

    let res = serde_wasm_bindgen::from_value(res).map_err(|e| e.to_string())?;
    Ok(res)
}

pub async fn get_institution(auth_key: &str, id: Option<String>) -> Result<Institution, String> {
    let res = invokeGetInstitution(auth_key, id).await;
    match res {
        Ok(j) => Ok(serde_wasm_bindgen::from_value(j).unwrap()),
        Err(e) => Err(e.as_string().unwrap()),
    }
}

pub async fn transactions_sync(auth_key: &str, access_token: &str) -> Result<(), String> {
    invokeTransactionsSync(auth_key, access_token)
        .await
        .map_err(|e| e.as_string().unwrap())
}

pub async fn get_balances(auth_token: &str, user_id: &str) -> Result<Vec<Account>, String> {
    let res = invokeGetPlaidBalances(auth_token, user_id).await;
    match res {
        Ok(json) => {
            log::info!("{:?}", json);
            Ok(serde_wasm_bindgen::from_value(json).map_err(|e| e.to_string())?)
        }
        Err(e) => {
            log::error!("{:?}", e);
            Err(e.as_string().expect("Failed to get string"))
        }
    }
}
