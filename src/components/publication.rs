use dioxus::prelude::*;

#[component]
pub fn Publication(title: String, link: String, description: String) -> Element {
    rsx! {
        div { class: "project",
            div { class: "project-title",
                h3 { title }
                div { class: "project-gh",
                    a { class: "media-icon", href: link,
                        span { class: "fa fa-github" }
                    }
                }
            }
            div { class: "project-desc",
                p { "{description}" }
            }
        }
    }
}
