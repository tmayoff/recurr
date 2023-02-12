use chrono::{Local, Months, NaiveDate};
use recurr_core::{Account, Category, SupabaseAuthCredentials, TransactionOption, Transactions};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn invokeGetSupbaseAuthCredentials() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeLinkTokenCreate(anon_key: &str, user_id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeItemPublicTokenExchange(
        anon_key: &str,
        public_token: &str,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeGetCategories() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeGetTransactions(
        auth_key: &str,
        access_token: &str,
        start_date: String,
        end_date: String,
        options: JsValue,
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

    #[wasm_bindgen(catch)]
    pub async fn invokeGetPlaidBalances(
        auth_token: &str,
        user_id: &str,
    ) -> Result<JsValue, JsValue>;

    pub fn linkStart(link_token: String, callback: JsValue);
}

pub async fn get_supabase_auth_credentials() -> Result<SupabaseAuthCredentials, String> {
    let res = invokeGetSupbaseAuthCredentials().await;
    match res {
        Ok(json) => {
            let obj = serde_wasm_bindgen::from_value::<SupabaseAuthCredentials>(json)
                .map_err(|e| e.to_string())?;
            Ok(obj)
        }
        Err(e) => Err(e.as_string().expect("Failed to get string")),
    }
}

pub async fn get_categories() -> Result<Vec<Category>, String> {
    let res = invokeGetCategories()
        .await
        .map_err(|e| e.as_string().unwrap())?;

    let res = serde_wasm_bindgen::from_value(res).map_err(|e| e.to_string())?;
    Ok(res)
}

pub async fn get_transactions(
    auth_key: &str,
    access_token: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    options: TransactionOption,
) -> Result<Transactions, String> {
    let start_date = start_date
        .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap())
        .format("%Y-%m-%d")
        .to_string();
    let end_date = end_date
        .unwrap_or(
            Local::now()
                .date_naive()
                .checked_add_months(Months::new(1))
                .expect("Date must be valid"),
        )
        .format("%Y-%m-%d")
        .to_string();

    let res = invokeGetTransactions(
        auth_key,
        access_token,
        start_date,
        end_date,
        serde_wasm_bindgen::to_value(&options).expect("Failed to serialize"),
    )
    .await
    .map_err(|e| e.as_string().unwrap())?;

    let res = serde_wasm_bindgen::from_value(res).map_err(|e| e.to_string())?;
    Ok(res)
}

pub async fn get_balances(auth_token: &str, user_id: &str) -> Result<Vec<Account>, String> {
    let res = invokeGetPlaidBalances(auth_token, user_id).await;
    match res {
        Ok(json) => Ok(serde_wasm_bindgen::from_value(json).map_err(|e| e.to_string())?),
        Err(e) => Err(e.as_string().expect("Failed to get string")),
    }
}

pub async fn get_all_accounts(
    auth_token: &str,
    user_id: &str,
) -> Result<Vec<(recurr_core::Institution, Vec<recurr_core::Account>)>, String> {
    let res = invokeGetPlaidAccounts(auth_token, user_id).await;
    match res {
        Ok(accounts) => {
            let accounts =
                serde_wasm_bindgen::from_value(accounts).expect("Failed to deserialize data");
            Ok(accounts)
        }
        Err(e) => {
            log::error!("{:?}", e);
            Err(e.as_string().expect("Failed to get string"))
        }
    }
}
