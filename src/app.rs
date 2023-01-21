use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::auth::Auth;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <div class="row">
                <Auth />
            </div>
        </main>
    }
}
