use recurr_core::get_supbase_client;

use crate::plaid::item_remove;

#[tauri::command]
pub async fn remove_account(
    user_id: &str,
    auth_key: &str,
    access_token: &str,
) -> Result<(), String> {
    item_remove(auth_key, access_token)
        .await
        .map_err(|e| e.to_string())?;

    let client = get_supbase_client();
    let _ = client
        .from("access_tokens")
        .auth(auth_key)
        .eq("user_id", user_id)
        .eq("access_token", access_token)
        .delete()
        .execute()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;

    Ok(())
}
