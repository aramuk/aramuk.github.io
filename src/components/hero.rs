use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero(title: String, subtitles: Vec<String>, image: String) -> Element {
    rsx! {
        div {
            id: "hero",
            class: "hero",
            div { class: "hero-title",
                div { class: "hero-title-box",
                    h3 { "{title}" }
                    {subtitles.iter().map(|s| rsx!{ 
                        p { "{s}" }
                    })}
                }
            }
            img { src: "{image}"}
        }
    }
}
