use supabase_js_rs::{Credentials, SupabaseClient};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub client: SupabaseClient,
    pub data: JsValue,
}

impl Default for Session {
    fn default() -> Self {
        let client = supabase_js_rs::create_client(
            "https://linaejyblplchxcrusjy.supabase.co",
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImxpbmFlanlibHBsY2h4Y3J1c2p5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzQyNjc3ODMsImV4cCI6MTk4OTg0Mzc4M30.CSc7E2blxAaO2ijXxOGjmhdgmlDVKmBAUSROuWPujWI",
        );

        Self {
            client,
            data: JsValue::NULL,
        }
    }
}

impl Session {
    pub async fn sign_up(&self, email: &str, password: &str) -> Result<JsValue, JsValue> {
        let res = self
            .client
            .auth()
            .sign_up(Credentials {
                email: email.to_string(),
                password: password.to_string(),
            })
            .await;

        Ok(res.unwrap())
    }
}

#[tauri::command]
pub async fn sign_up(email: &str, password: &str) -> Result<(), String> {
    log::info!("Singup");
    Ok(())
}
