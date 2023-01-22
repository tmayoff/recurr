#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plaid;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            plaid::link_token_create,
            plaid::item_public_token_exchange
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
