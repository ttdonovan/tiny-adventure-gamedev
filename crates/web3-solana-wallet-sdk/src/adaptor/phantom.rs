use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = solana)]
    pub static SOLANA: Solana;

    #[wasm_bindgen]
    pub type Solana;

    #[wasm_bindgen(js_namespace = solana, js_name = connect)]
    pub fn connect() -> Promise;

    #[wasm_bindgen(js_namespace = solana, js_name = disconnect)]
    pub fn disconnect() -> Promise;

    #[wasm_bindgen(method, getter, js_name = publicKey)]
    pub fn public_key(this: &Solana) -> JsValue;

    #[wasm_bindgen(js_namespace = solana, js_name = signIn)]
    pub fn sign_in(options: &JsValue) -> Promise;
}
