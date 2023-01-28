use std::collections::HashMap;

use postgrest::Postgrest;
use recurr_core::{Account, Institution};

use crate::{
    plaid::accounts::accounts_get,
    supabase::{access_token::get_access_token, SchemaAccessToken, SchemaPlaidAccount},
};

#[tauri::command]
pub async fn get_plaid_accounts(
    auth_token: &str,
    user_id: &str,
) -> Result<Vec<(Institution, Vec<Account>)>, super::Error> {
    let client = Postgrest::new("https://linaejyblplchxcrusjy.supabase.co/rest/v1")
    .insert_header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImxpbmFlanlibHBsY2h4Y3J1c2p5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzQyNjc3ODMsImV4cCI6MTk4OTg0Mzc4M30.CSc7E2blxAaO2ijXxOGjmhdgmlDVKmBAUSROuWPujWI");

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
    let client = Postgrest::new("https://linaejyblplchxcrusjy.supabase.co/rest/v1")
        .insert_header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImxpbmFlanlibHBsY2h4Y3J1c2p5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzQyNjc3ODMsImV4cCI6MTk4OTg0Mzc4M30.CSc7E2blxAaO2ijXxOGjmhdgmlDVKmBAUSROuWPujWI");

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
