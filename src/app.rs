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

    #[wasm_bindgen(js_name = link_start)]
    pub async fn link_start(link_token: String);
}

#[function_component(App)]
pub fn app() -> Html {
    let link = |_| {
        spawn_local(async move {
            let link = link_create().await;
            // log::info!("{:?}", link.unwrap());

            match link.unwrap().as_string() {
                Some(link_token) => {
                    link_start(link_token).await;
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
