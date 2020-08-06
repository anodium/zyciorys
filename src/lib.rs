#[path = "app.rs"]
mod app;
pub(crate) use self::app::*;

#[path = "resume/mod.rs"]
pub(crate) mod resume;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

    console_error_panic_hook::set_once();

    yew::start_app::<App>();

    Ok(())
}
