use postgrest::Postgrest;
use recurr_core::Error;

pub mod access_token;
pub mod accounts;

pub fn get_supbase_client() -> Result<Postgrest, Error> {
    let client = Postgrest::new(env!("SUPABASE_URL").to_owned() + "/rest/v1")
        .insert_header("apikey", env!("SUPABASE_KEY"));

    Ok(client)
}
