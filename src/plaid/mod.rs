use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::{
    function_component, html,
    platform::{pinned::oneshot, spawn_local},
    use_context, Html,
};

use crate::context::SessionContext;

#[derive(Serialize, Debug)]
pub struct User {
    pub client_user_id: String,
}

#[derive(Serialize, Debug)]
pub struct LinkTokenCreateRequest {
    pub client_name: String,
    pub language: String,
    pub country_codes: Vec<String>,
    pub products: Vec<String>,
    pub user: User,
}

impl LinkTokenCreateRequest {
    pub fn new(
        client_name: &str,
        language: &str,
        country_codes: Vec<String>,
        products: Vec<String>,
        user: User,
    ) -> Self {
        Self {
            client_name: client_name.to_string(),
            language: language.to_string(),
            country_codes: country_codes.to_vec(),
            products: products,
            user: user,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct LinkTokenCreateReponse {
    expiration: String,
    link_token: String,
    request_id: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn invokeLinkTokenCreate(anon_key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeItemPublicTokenExchange(
        anon_key: &str,
        public_token: &str,
    ) -> Result<JsValue, JsValue>;

    pub fn linkStart(link_token: String, callback: JsValue);
}

pub async fn link_token_create(anon_key: &str) -> Result<LinkTokenCreateReponse, String> {
    let response = invokeLinkTokenCreate(anon_key).await;

    match response {
        Ok(response) => {
            let res = serde_wasm_bindgen::from_value::<LinkTokenCreateReponse>(response)
                .expect("Response not valid");
            Ok(res)
        }
        Err(e) => Err(e.as_string().unwrap()),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Success {
    public_token: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct Failure {
    err: String,
}

fn link_start(link_token: String, mut callback: impl FnMut(Result<Success, Failure>) + 'static) {
    linkStart(
        link_token,
        Closure::once_into_js(move |response: JsValue| {
            let s = serde_wasm_bindgen::from_value::<Success>(response.clone());

            if let Ok(success) = s {
                callback(Ok(success));
                return;
            };

            let e = serde_wasm_bindgen::from_value::<Failure>(response.clone());
            if let Ok(failure) = e {
                callback(Err(failure));
                return;
            };
        }),
    );
}

#[function_component(Link)]
pub fn link() -> Html {
    let context = use_context::<SessionContext>().expect("No context");

    let link = move |_| {
        let context = context.clone();
        spawn_local(async move {
            let context = context.clone();
            let response = link_token_create(&context.anon_key).await;
            let link_token;
            match response {
                Ok(res) => {
                    link_token = res.link_token;
                }
                Err(e) => {
                    log::error!("{:?}", e);
                    return;
                }
            }

            let (tx, rx) = oneshot::channel::<Result<Success, Failure>>();

            let sender_mtx = Mutex::new(Some(tx));

            link_start(link_token, move |response| {
                if let Some(tx) = sender_mtx.lock().unwrap().take() {
                    let _ = tx.send(response);
                }
            });

            let response = rx.await;

            if let Ok(response) = response {
                match response {
                    Ok(success) => {
                        let res =
                            invokeItemPublicTokenExchange(&context.anon_key, &success.public_token)
                                .await;
                        log::info!("{:?}", res);
                    }
                    Err(error) => log::error!("{:?}", error),
                }
            };
        })
    };

    html! {
        <main class="container">
            <script src="https://cdn.plaid.com/link/v2/stable/link-initialize.js"></script>
            <div class="row">
                <button type="button" onclick={link}>{"Link"}</button>
            </div>
        </main>
    }
}
