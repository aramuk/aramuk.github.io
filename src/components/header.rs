use dioxus::prelude::*;

#[component]
pub fn Header(scrolled: bool) -> Element {
    let bg_class = if scrolled { " bg" } else { "" };
    rsx! {
        header {
            nav { class: "navbar {bg_class}",
                ul {
                    li { class: "nav-link {bg_class}",
                        a { class: "title", href: "/",
                            h1 { class: "nav-link-text", "Aditesh Kumar" }
                        }
                    }
                    li { class: "nav-link {bg_class}",
                        a { href: "#about",
                            h2 { class: "nav-link-text", "About" }
                        }
                    }
                    li { class: "nav-link {bg_class}",
                        a { href: "#pubs",
                            h2 { class: "nav-link-text", "Publications" }

                        }
                    }
                }
            }
        }
    }
}
