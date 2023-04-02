use crate::{
    auth::AuthComponent,
    context::{ContextUpdate, Session, SessionProvider},
    dashboard::Dashboard,
    supabase,
};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Error;
use supabase_js_rs::SupabaseClient;

use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};
use yew::{platform::spawn_local, prelude::*};

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {

    #[wasm_bindgen]
    pub fn setEventListener(callback: &JsValue);
}

fn tauri_event_handler(context: &UseReducerHandle<Session>) {
    let client = context.supabase_client.clone();

    setEventListener(&Closure::once_into_js(move |e: JsValue| {
        #[derive(Deserialize)]
        struct Event {
            event: String,
            payload: recurr_core::Event,
        }

        let event: Event = serde_wasm_bindgen::from_value(e).expect("Failed to deserialize");

        match event.payload {
            recurr_core::Event::DeepLink(link) => {
                #[derive(Serialize)]
                struct Params {
                    email: String,
                    token: String,
                    #[serde(rename = "type")]
                    verify_type: String,
                }

                let link = link.replacen("#", "?", 1);
                let url = url::Url::parse(&link).expect("Failed to parse url");
                let mut query_pairs = url.query_pairs();
                let access_token = query_pairs.next().unwrap().1.to_string();
                let expires_in = query_pairs.next().unwrap().1.to_string();
                let refresh_token = query_pairs.next().unwrap().1.to_string();

                spawn_local(async move {
                    client
                        .auth()
                        .set_session(supabase_js_rs::CurrentSession {
                            access_token,
                            refresh_token,
                        })
                        .await;
                });
            }
        }
    }));
}

fn setup_auth_handler(context: &UseReducerHandle<Session>, client: &SupabaseClient) {
    let callback_context = context.clone();
    let auth_callback: Closure<dyn FnMut(JsValue, JsValue)> =
        Closure::new(move |_: JsValue, session: JsValue| {
            log::debug!("Updated Session {:?}", session);
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
        tauri_event_handler(&context);

        let client = context.supabase_client.clone();

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
                    <AuthComponent context={context.clone()}/>
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
