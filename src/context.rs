use crate::supabase;
use supabase_js_rs::SupabaseClient;

#[derive(Clone, Debug, PartialEq)]
pub struct SessionContext {
    pub supabase_client: SupabaseClient,
    pub supabase_session: Option<supabase::Session>,
}
