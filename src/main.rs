use dioxus::prelude::*;

mod components;
mod pages;

use components::navbar::*;
use pages::blog::*;
use pages::home::*;
use pages::not_found::*;

const INDEX_CSS: Asset = asset!("assets/css/index.css");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    // Handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "icon",
            href: "data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%2210 0 100 100%22><text y=%22.90em%22 font-size=%2290%22>🌉</text></svg>"
        }
        document::Link { rel: "stylesheet", href: INDEX_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Meta { name: "description", content: "My academic portfolio." }
        document::Meta { name: "author", content: "Aditesh Kumar" }
        document::Title { "Aditesh Kumar" }
        Router::<Route> {}
    }
}
