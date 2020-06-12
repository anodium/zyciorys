mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    yew::start_app::<app::App>();

    Ok(())
}
