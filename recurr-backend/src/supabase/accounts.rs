use postgrest::Postgrest;

use crate::supabase::{access_token::get_access_token, SchemaPlaidAccount};

use super::SupabaseErrors;

#[tauri::command]
pub async fn get_plaid_accounts(
    auth_token: &str,
    user_id: &str,
    access_token: &str,
) -> Result<Vec<String>, SupabaseErrors> {
    log::info!("Get Plaid Accounts");

    let client = Postgrest::new("https://linaejyblplchxcrusjy.supabase.co/rest/v1")
    .insert_header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImxpbmFlanlibHBsY2h4Y3J1c2p5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzQyNjc3ODMsImV4cCI6MTk4OTg0Mzc4M30.CSc7E2blxAaO2ijXxOGjmhdgmlDVKmBAUSROuWPujWI");

    let access_token_row = get_access_token(&auth_token, &user_id, &access_token).await?;
    log::info!("Access Token Row {:?}", access_token_row);

    // let body = serde_json::to_string(&SchemaPlaidAccount {
    //     user_id: user_id.to_owned(),
    //     access_token_id: access_token_row.id,
    // })
    // .expect("Failed to serialize schema");

    let res = client
        .from("plaid_accounts")
        .auth(auth_token)
        .select("*,access_tokens(*)")
        .eq("user_id", user_id)
        .execute()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                log::info!("{:?}", res.text().await);
                return Ok(Vec::new());
            } else {
                return Err(SupabaseErrors::RequestError(
                    res.text().await.expect("Failed to stringify"),
                ));
            }
        }
        Err(err) => return Err(SupabaseErrors::RequestError(err.to_string())),
    }
}

#[tauri::command]
pub async fn save_plaid_account(
    auth_token: &str,
    user_id: &str,
    access_token: &str,
    plaid_account_id: &str,
) -> Result<(), SupabaseErrors> {
    let client = Postgrest::new("https://linaejyblplchxcrusjy.supabase.co/rest/v1")
        .insert_header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImxpbmFlanlibHBsY2h4Y3J1c2p5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzQyNjc3ODMsImV4cCI6MTk4OTg0Mzc4M30.CSc7E2blxAaO2ijXxOGjmhdgmlDVKmBAUSROuWPujWI");

    let access_token_row = get_access_token(&auth_token, &user_id, &access_token).await?;
    log::info!("Access Token Row {:?}", access_token_row);

    let body = serde_json::to_string(&SchemaPlaidAccount {
        user_id: user_id.to_owned(),
        account_id: plaid_account_id.to_owned(),
        access_token_id: access_token_row.id,
    })
    .expect("Failed to serialize schema");

    let res = client
        .from("plaid_accounts")
        .auth(auth_token)
        .insert(&body)
        .execute()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                return Ok(());
            } else {
                return Err(SupabaseErrors::RequestError(
                    res.text().await.expect("Failed to stringify"),
                ));
            }
        }
        Err(err) => return Err(SupabaseErrors::RequestError(err.to_string())),
    }
}
