#![warn(clippy::all, rust_2018_idioms)]
mod app;
mod filters;
mod spell;
mod util;
pub use app::SpellSearchApp;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn update_avail() {
    if let Ok(mut update_available) = app::UPDATE_AVAILABLE.lock() {
        *update_available = true;
    }
}
