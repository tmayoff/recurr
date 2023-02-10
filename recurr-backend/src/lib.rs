use tauri::App;

mod plaid;
mod supabase;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn run(self) {
        let setup = self.setup;
        tauri::Builder::default()
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                plaid::accounts::get_balances,
                // plaid::link::link_token_create,
                // plaid::item_public_token_exchange,
                // supabase::access_token::save_access_token,
                // supabase::access_token::get_access_token,
                // supabase::accounts::save_plaid_account,
                // supabase::accounts::get_plaid_accounts,
                // supabase::auth::get_supabase_auth_credentials,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}