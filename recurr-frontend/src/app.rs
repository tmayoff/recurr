use crate::{
    auth::Auth,
    commands,
    context::{Session, SessionContext, SessionProvider},
    dashboard::Dashboard,
    supabase,
};
use serde_wasm_bindgen::Error;
use supabase_js_rs::SupabaseClient;
use wasm_bindgen::{prelude::Closure, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

fn setup_auth_handler(context: &UseReducerHandle<Session>, client: &SupabaseClient) {
    let callback_context = context.clone();
    let auth_callback: Closure<dyn FnMut(JsValue, JsValue)> =
        Closure::new(move |_: JsValue, session: JsValue| {
            let session: Result<supabase::Session, Error> = serde_wasm_bindgen::from_value(session);
            if let Ok(session) = session {
                callback_context.dispatch((Some(session), None));
            } else {
                callback_context.dispatch((None, None));
            }
        });

    client.auth().on_auth_state_change(&auth_callback);
    auth_callback.forget();
}

#[function_component(Main)]
fn main() -> Html {
    let context = use_context::<SessionContext>().unwrap();
    let use_context = context.clone();

    let async_context = context.clone();
    spawn_local(async move {
        let cred = commands::get_supabase_auth_credentials().await;

        let cred = cred.expect("Failed to get credentials");
        let client = supabase_js_rs::create_client(&cred.auth_url, &cred.anon_key);
        setup_auth_handler(&async_context, &client);
        async_context.dispatch((None, Some(client)));
    });

    let has_session = use_context.supabase_session.is_some();

    html! {
        <main class="hero is-fullheight">
        {
            if context.supabase_client.is_some() {
                html!{
                    if has_session {
                        <Dashboard />
                    } else {
                        <Auth />
                    }
                }
            } else {
                html!{}
            }
        }
        </main>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <SessionProvider>
            <Main/>
        </SessionProvider>
    }
}
