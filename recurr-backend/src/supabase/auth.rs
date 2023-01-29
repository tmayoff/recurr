use recurr_core::SupabaseAuthCredentials;

#[tauri::command]
pub fn get_supabase_auth_credentials() -> Result<SupabaseAuthCredentials, String> {
    Ok(SupabaseAuthCredentials {
        auth_url: std::env::var("SUPABASE_BASE_URL").map_err(|e| e.to_string())?,
        anon_key: std::env::var("SUPABASE_KEY").map_err(|e| e.to_string())?,
    })
}
