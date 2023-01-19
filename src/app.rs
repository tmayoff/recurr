use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeLinkCreate, catch)]
    pub async fn link_create() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invokeTokenExchange, catch)]
    pub async fn token_exchange(public_token: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = link_start)]
    pub async fn link_start(link_token: String) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    let link = |_| {
        spawn_local(async move {
            let link = link_create().await;

            match link.unwrap().as_string() {
                Some(link_token) => {
                    let public_token = link_start(link_token).await;
                    let res = token_exchange(public_token.as_string().unwrap_or_default()).await;
                    match res {
                        Ok(access_token) => {
                            log::info!("{:?}", access_token);
                        }
                        Err(err) => {
                            log::error!("{:?}", err);
                        }
                    }
                }
                None => {}
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
