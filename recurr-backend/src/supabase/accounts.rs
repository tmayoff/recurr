use std::collections::HashMap;

use recurr_core::{Account, Institution};

use crate::{
    plaid::{self, accounts::accounts_get},
    supabase::{access_token::get_access_token, SchemaAccessToken, SchemaPlaidAccount},
};

#[tauri::command]
pub async fn get_plaid_balances(
    auth_key: &str,
    user_id: &str,
) -> Result<Vec<Account>, super::Error> {
    // Get Account IDs and access tokens from supabase
    let client = super::get_supbase_client()?;
    let res = client
        .from("access_tokens")
        .auth(auth_key)
        .select("*,plaid_accounts(*)")
        .eq("user_id", user_id)
        .execute()
        .await?
        .error_for_status()?;

    let access_tokens: Vec<SchemaAccessToken> = res.json().await?;

    let mut all_accounts = Vec::new();
    for access_token in access_tokens {
        if let Some(token) = access_token.plaid_accounts {
            let account_ids = token
                .into_iter()
                .map(|a| a.account_id)
                .collect::<Vec<String>>();

            let accounts =
                plaid::accounts::get_balances(auth_key, &access_token.access_token, account_ids)
                    .await?;
            all_accounts.extend(accounts);
        }
    }

    Ok(all_accounts)
}

#[tauri::command]
pub async fn get_plaid_accounts(
    auth_token: &str,
    user_id: &str,
) -> Result<Vec<(Institution, Vec<Account>)>, super::Error> {
    let client = super::get_supbase_client()?;

    // Get access tokens and their associated plaid accounts
    let res = client
        .from("access_tokens")
        .auth(auth_token)
        .select("*,plaid_accounts(*)")
        .eq("user_id", user_id)
        .execute()
        .await?
        .error_for_status()?;

    let mut token_account_ids = HashMap::new();

    let access_tokens: Vec<SchemaAccessToken> = res.json().await?;

    for token in access_tokens {
        // let mut account_ids = token
        let mut account_ids = Vec::new();
        if let Some(accounts) = token.plaid_accounts {
            for account in accounts {
                account_ids.push(account.account_id);
            }
        }

        if !account_ids.is_empty() {
            token_account_ids.insert(token.access_token, account_ids);
        }
    }

    let mut all_accounts = Vec::new();
    for token_account_id in token_account_ids {
        let accounts = accounts_get(auth_token, &token_account_id.0, token_account_id.1).await?;
        all_accounts.push(accounts);
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
    let client = super::get_supbase_client()?;

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
        .await?
        .error_for_status()?;

    Ok(())
}
