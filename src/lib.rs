#[path = "app.rs"]
mod app;
pub use self::app::*;

#[path = "resume.rs"]
pub mod resume;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

    console_error_panic_hook::set_once();

    yew::start_app::<app::App>();

    Ok(())
}
