use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_context, Html};

use crate::{commands::get_all_accounts, context::SessionContext};

#[function_component(Accounts)]
pub fn accounts() -> Html {
    let context = use_context::<SessionContext>().expect("Requires context");

    let session = context
        .supabase_session
        .as_ref()
        .expect("Requires supabase session");
    let user_id = session.user.id.clone();
    let access_token = session.access_token.clone();

    spawn_local(async move {
        get_all_accounts(&access_token, &user_id).await;
    });

    html! {
        <div>
        {"Accounts here"}
        </div>
    }
}
