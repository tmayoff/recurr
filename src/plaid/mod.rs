use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::{platform::pinned::oneshot, prelude::*};

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
    pub async fn invokeLinkCreate() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invokeTokenExchange(public_token: String) -> Result<JsValue, JsValue>;

    pub fn linkStart(link_token: String, callback: JsValue);
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
    let link = |_| {
        spawn_local(async move {
            let link = invokeLinkCreate().await;

            let link_token = link
                .expect("Link Failed")
                .as_string()
                .expect("Response not a string");

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
                        let res = invokeTokenExchange(String::from(success.public_token)).await;
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
