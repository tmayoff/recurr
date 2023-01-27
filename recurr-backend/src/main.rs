#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plaid;
mod supabase;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            plaid::link::link_token_create,
            plaid::item_public_token_exchange,
            plaid::accounts_balance_get,
            supabase::access_token::save_access_token,
            supabase::access_token::get_access_token,
            supabase::accounts::save_plaid_account,
            supabase::accounts::get_plaid_accounts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
