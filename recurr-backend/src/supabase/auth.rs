use recurr_core::SupabaseAuthCredentials;

#[tauri::command]
pub fn get_supabase_auth_credentials() -> Result<SupabaseAuthCredentials, String> {
    Ok(SupabaseAuthCredentials {
        auth_url: env!("SUPABASE_URL").to_string(),
        anon_key: env!("SUPABASE_KEY").to_string(),
    })
}
