use dioxus::prelude::*;

#[component]
pub fn Hero(title: String, subtitles: Vec<String>, image: String) -> Element {
    rsx! {
        div { class: "hero",
            div {
                style: "background-image: url({image});",
                class: "bg-cover bg-fixed bg-hero hero-image",
                div { class: "hero-title",
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
