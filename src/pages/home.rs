use dioxus::prelude::*;

use crate::components::hero::*;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        header {
            nav { class: "navigation bg",
                ul {
                    li { class: "section-link bg",
                        a { class: "title", href: "/",
                            h1 { "Aditesh Kumar" }
                        }
                    }
                    li { class: "section-link bg",
                        a { href: "#about",
                            h3 { "About" }
                        }
                    }
                    li { class: "section-link bg",
                        a { href: "#pubs",
                            h3 { "Publications" }

                        }
                    }
                }
            }
        }
        main {
            Hero {
                title: "Currently:",
                subtitles: vec![
                    "MSCS @ Stanford".to_string(),
                    "Research Assistant @ SVL".to_string(),
                    "Course Assistant @ CS 231N".to_string()
                ],
                image: asset!("/assets/img/lake_tahoe.jpeg"),
            }
        }
        footer {
            {"Copyright 2020-2025 Aditesh Kumar"}
        }
    }
}
