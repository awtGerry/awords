mod app;
pub mod views;
pub mod components;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::app::*;
        use leptos::*;

        #[wasm_bindgen]
        pub fn hydrate() {
                console_error_panic_hook::set_once();
                _ = console_log::init_with_level(log::Level::Debug);

                leptos::mount_to_body(App);
        }
    }
}
