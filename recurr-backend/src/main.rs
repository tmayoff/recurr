#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod plaid;
mod supabase;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            let window = app.get_window("main");
            if let Some(window) = window {
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            plaid::link::link_token_create,
            plaid::item_public_token_exchange,
            plaid::accounts::balance_get,
            supabase::auth::get_supabase_auth_credentials,
            supabase::access_token::save_access_token,
            supabase::access_token::get_access_token,
            supabase::accounts::save_plaid_account,
            supabase::accounts::get_plaid_accounts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
