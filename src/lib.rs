#![recursion_limit = "128"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;
mod pages;
mod components;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::Model>::new().mount_to_body();
}
