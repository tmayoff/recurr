use recurr_core::get_supbase_client;

use crate::plaid::item_remove;

#[tauri::command]
pub async fn remove_account(
    user_id: &str,
    auth_key: &str,
    access_token: &str,
) -> Result<(), recurr_core::Error> {
    item_remove(auth_key, access_token).await?;

    let client = get_supbase_client();
    _ = client
        .from("access_tokens")
        .auth(auth_key)
        .eq("user_id", user_id)
        .eq("access_token", access_token)
        .delete()
        .execute()
        .await
        .map(|r| r.error_for_status())
        .flatten()
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;

    Ok(())
}
