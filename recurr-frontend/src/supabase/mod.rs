use postgrest::Postgrest;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Session {
    #[serde(rename = "access_token")]
    pub auth_key: String,
    pub token_type: String,
    pub expires_in: u32,
    pub user: User,
}

pub fn get_supbase_client() -> Postgrest {
    Postgrest::new(env!("SUPABASE_URL").to_owned() + "/rest/v1")
        .insert_header("apikey", env!("SUPABASE_KEY"))
}
