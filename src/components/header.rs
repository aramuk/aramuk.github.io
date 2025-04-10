use dioxus::prelude::*;

#[component]
pub fn Header(scrolled: bool) -> Element {
    let bg_class = if scrolled { " bg" } else { "" };
    rsx! {
        header {
            nav { class: "navbar {bg_class}",
                ul {
                    li { class: "nav-link {bg_class}",
                        a {
                            class: "title", 
                            aria_label: "Jump to top of page.",
                            href: "#",
                            h1 { class: "nav-link-text", "Aditesh Kumar" }
                        }
                    }
                    li { class: "nav-link {bg_class}",
                        a { 
                            aria_label: "Jump to About section.",
                            href: "#about",
                            div { class: "nav-link-text", "About" }
                        }
                    }
                    li { class: "nav-link {bg_class}",
                        a { 
                            aria_label: "Jump to Publications section.",
                            href: "#pubs",
                            div { class: "nav-link-text", "Publications" }

                        }
                    }
                }
            }
        }
    }
}
