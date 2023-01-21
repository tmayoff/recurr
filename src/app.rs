use crate::{auth::Auth, context::SessionContext, supabase};
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let context = use_memo(
        |_| {
            let session =      SessionContext {supabase_client: supabase_js_rs::create_client(
        "https://linaejyblplchxcrusjy.supabase.co",
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImxpbmFlanlibHBsY2h4Y3J1c2p5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzQyNjc3ODMsImV4cCI6MTk4OTg0Mzc4M30.CSc7E2blxAaO2ijXxOGjmhdgmlDVKmBAUSROuWPujWI",
    ), supabase_session: None};
            let use_session = session.clone();
            spawn_local(async move {
                let res = use_session.supabase_client.auth().get_session().await;
                log::info!("{:?}", res);
            });

            session
        },
        (),
    );

    let use_context = context.clone();

    let auth_callback: Closure<dyn FnMut(JsValue, JsValue)> =
        Closure::new(move |data: JsValue, session: JsValue| {
            let session: supabase::Session = serde_wasm_bindgen::from_value(session).unwrap();
            log::info!("\t{:?}", session);
        });

    use_context
        .supabase_client
        .auth()
        .on_auth_state_change(&auth_callback);
    auth_callback.forget();

    let has_session = use_context.supabase_session.is_some();

    html! {
        <ContextProvider<Rc<SessionContext>> context={context}>
            <main class="full-height">
                if has_session {
                    <Auth />
                } else {
                    <div class="full-height columns">
                        <div class="column is-one-fifth has-background-info">
                            {"Sidebar"}
                        </div>
                        <div class="column">
                            {"Main Area"}
                        </div>
                    </div>
                }
            </main>
        </ContextProvider<Rc<SessionContext>>>
    }
}
