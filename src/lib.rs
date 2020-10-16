#![recursion_limit = "128"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;

use components::home::HomePage;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<HomePage>::new().mount_to_body();
}
