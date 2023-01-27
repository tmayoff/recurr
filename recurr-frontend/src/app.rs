use crate::{
    auth::Auth,
    context::{Session, SessionContext, SessionProvider},
    dashboard::Dashboard,
    supabase,
};
use serde_wasm_bindgen::Error;
use wasm_bindgen::{prelude::Closure, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

fn setup_auth_handler(context: UseReducerHandle<Session>) {
    let callback_context = context.clone();
    let auth_callback: Closure<dyn FnMut(JsValue, JsValue)> =
        Closure::new(move |_: JsValue, session: JsValue| {
            let session: Result<supabase::Session, Error> = serde_wasm_bindgen::from_value(session);
            if let Ok(session) = session {
                callback_context.dispatch(Some(session));
            } else {
                callback_context.dispatch(None);
            }
        });

    let use_context = context.clone();
    spawn_local(async move {
        let session = use_context.supabase_client.auth().get_session().await;
    });

    context
        .supabase_client
        .auth()
        .on_auth_state_change(&auth_callback);
    auth_callback.forget();
}

#[function_component(Main)]
fn main() -> Html {
    let context = use_context::<SessionContext>().unwrap();
    let use_context = context.clone();

    setup_auth_handler(context);

    let has_session = use_context.supabase_session.is_some();

    html! {
        <main class="hero is-fullheight">
        if has_session {
            <Dashboard />
        } else {
            <Auth />
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
