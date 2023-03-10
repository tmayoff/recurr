#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod accounts;
mod plaid;
mod supabase;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main");
                if let Some(window) = window {
                    window.open_devtools();
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            accounts::remove_account,
            plaid::link::link_token_create,
            plaid::accounts::get_accounts,
            plaid::institutions::get_institution,
            plaid::transactions::get_transactions,
            plaid::transactions::get_categories,
            plaid::item_public_token_exchange,
            supabase::access_token::save_access_token,
            supabase::access_token::get_access_tokens,
            // supabase::access_token::get_access_token,
            supabase::accounts::save_plaid_account,
            supabase::accounts::get_plaid_balances,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
