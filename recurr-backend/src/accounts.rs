use crate::{plaid::item_remove, supabase::get_supbase_client};

#[tauri::command]
pub async fn remove_account(
    user_id: &str,
    auth_key: &str,
    access_token: &str,
) -> Result<(), String> {
    item_remove(auth_key, access_token)
        .await
        .map_err(|e| e.to_string())?;

    let client = get_supbase_client().map_err(|e| e.to_string())?;
    let res = client
        .from("access_tokens")
        .eq("user_id", user_id)
        .eq("access_token", access_token)
        .delete()
        .execute()
        .await
        .map_err(|e| e.to_string());

    log::info!("{:?}", res);

    Ok(())
}
