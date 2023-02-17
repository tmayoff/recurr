#[tauri::command]
pub fn delete_account() -> Result<(), String> {
    log::info!("Delete account");

    Ok(())
}
