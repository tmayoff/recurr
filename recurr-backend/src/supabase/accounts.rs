use futures::future;
use recurr_core::{get_supbase_client, Account, SchemaAccessToken, SchemaPlaidAccount};

use crate::{plaid, supabase::access_token::get_access_token};

#[tauri::command]
pub async fn get_plaid_balances(
    auth_key: &str,
    user_id: &str,
) -> Result<Vec<Account>, recurr_core::Error> {
    // Get Account IDs and access tokens from supabase
    let client = get_supbase_client();
    let res = client
        .from("access_tokens")
        .auth(auth_key)
        .select("*,plaid_accounts(*)")
        .eq("user_id", user_id)
        .execute()
        .await
        .and_then(|e| e.error_for_status())
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;

    let access_tokens: Vec<SchemaAccessToken> = res
        .json()
        .await
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;

    let mut futures = Vec::new();
    for access_token in access_tokens {
        if let Some(token) = access_token.plaid_accounts {
            let account_ids = token
                .into_iter()
                .map(|a| a.account_id)
                .collect::<Vec<String>>();

            futures.push(plaid::accounts::get_balances(
                auth_key,
                access_token.access_token,
                account_ids,
            ));
        }
    }

    let results = future::join_all(futures).await;
    let mut all_accounts = Vec::new();
    for res in results {
        all_accounts.extend(res?);
    }

    Ok(all_accounts)
}

#[tauri::command]
pub async fn save_plaid_account(
    auth_token: &str,
    user_id: &str,
    access_token: &str,
    plaid_account_id: &str,
) -> Result<(), super::Error> {
    let client = get_supbase_client();

    let access_token_row = get_access_token(auth_token, user_id, access_token).await?;

    let body = serde_json::to_string(&SchemaPlaidAccount {
        user_id: user_id.to_owned(),
        account_id: plaid_account_id.to_owned(),
        access_token_id: access_token_row.id,
    })?;

    let _ = client
        .from("plaid_accounts")
        .auth(auth_token)
        .insert(&body)
        .execute()
        .await
        .and_then(|res| res.error_for_status())
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;

    Ok(())
}
