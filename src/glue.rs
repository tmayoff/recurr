#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn invoke() -> Result<JsValue, JsValue>;
}
