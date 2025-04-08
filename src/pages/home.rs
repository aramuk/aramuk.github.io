use dioxus::prelude::*;

use crate::components::hero::*;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}

    }
}
