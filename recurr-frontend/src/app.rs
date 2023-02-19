use crate::{
    auth::Auth,
    context::{ContextUpdate, Session, SessionProvider},
    dashboard::Dashboard,
    supabase,
};
use serde_wasm_bindgen::Error;
use supabase_js_rs::SupabaseClient;
use wasm_bindgen::{prelude::Closure, JsValue};
use yew::prelude::*;

fn setup_auth_handler(context: &UseReducerHandle<Session>, client: &SupabaseClient) {
    let callback_context = context.clone();
    let auth_callback: Closure<dyn FnMut(JsValue, JsValue)> =
        Closure::new(move |_: JsValue, session: JsValue| {
            let session: Result<Option<supabase::Session>, Error> =
                serde_wasm_bindgen::from_value(session);
            match session {
                Ok(session) => callback_context.dispatch(ContextUpdate::Session(session)),
                Err(e) => {
                    log::error!("Auth status changed, but failed {} ", e);
                    callback_context.dispatch(ContextUpdate::Session(None));
                }
            }
        });

    client.auth().on_auth_state_change(&auth_callback);
    auth_callback.forget();
}

enum MainMessage {
    ContextUpdated(UseReducerHandle<Session>),
}

#[derive(Properties, PartialEq)]
struct MainProps;

struct Main {
    context: UseReducerHandle<Session>,
    _context_listener: ContextHandle<UseReducerHandle<Session>>,
}

impl Component for Main {
    type Message = MainMessage;
    type Properties = MainProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (context, context_listener) = ctx
            .link()
            .context(ctx.link().callback(MainMessage::ContextUpdated))
            .expect("No Context Provided");

        setup_auth_handler(&context, &context.supabase_client);

        Self {
            context,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MainMessage::ContextUpdated(context) => {
                self.context = context;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let context = &self.context;
        let has_session = self.context.supabase_session.is_some();

        html! {
            <main class="hero is-fullheight">
                if has_session {
                    <Dashboard context={context.clone()}/>
                } else {
                    <Auth />
                }
            </main>
        }
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
