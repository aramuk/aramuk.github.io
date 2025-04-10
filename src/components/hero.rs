use dioxus::prelude::*;

#[component]
pub fn Hero(title: String, subtitles: Vec<String>, image: String) -> Element {
    rsx! {
        div { class: "hero",
            div {
                aria_label: "Hero image: photo of a sunset taken at El Dorado Beach in Lake Tahoe.",
                style: "background-image: url({image});",
                class: "bg-cover bg-fixed bg-hero hero-image",
                div { 
                    aria_label: "Hero text: What I am currently up to.",
                    class: "hero-title",
                    div { class: "hero-title-box",
                        h3 { "{title}" }
                        {subtitles.iter().map(|s| rsx!{ 
                            p { "{s}" }
                        })}
                    }
                }
            }
        }
    }
}
