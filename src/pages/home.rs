// use tracing::event;
// use ::web_sys::window;
use dioxus::prelude::*;

use crate::components::header::*;
use crate::components::hero::*;
use crate::components::publication::*;
use crate::components::section::*;

const SCRIPT: &str = r#"'
document.addEventListener("scroll", e => {
    let scroll = window.scrollY;
    console.log(scroll);
    let scrolled = scroll > 0;
    for (const elem of document.getElementsByClassName("navbar")) {
        if (scrolled) {
            elem.classList.add("bg");
        } else {
        elem.classList.remove("bg");
        }
    }

    for (const elem of document.getElementsByClassName("nav-link")) {
        if (scrolled) {
            elem.classList.add("bg");
        } else {
        elem.classList.remove("bg");
        }
    }
    return scrolled;
});"#;

/// Home page
#[component]
pub fn Home() -> Element {
    // let mut scroll_y = use_signal(|| 0.);
    // let mut event_text = use_signal(|| String::new());
    // let mut input_element = use_signal(|| None);

    rsx! {
        Header { scrolled: true }
        main {
            id: "main",
            // onmounted: move |element| async move  {
            //     input_element.set(Some(element.data()));
            //     let vec = element.data().get_scroll_offset().await;
            //     event_text.set(format!("Scroll data is {}", vec.unwrap().y));
            // },
            // onscroll: move |_| async move {
            //     if let Some(main) = input_element() {
            //         let vec = main.get_scroll_offset().await;
            //         scroll_y.set(vec.unwrap().y);
            //     }
            // },
            Hero {
                title: "Currently:",
                subtitles: vec![
                    "MSCS @ Stanford".to_string(),
                    "RA @ SVL".to_string(),
                    "CA @ CS 231N".to_string()
                ],
                image: asset!("/assets/img/lake_tahoe.jpeg"),
            }
            Section {
                id: "about",
                title: "About",
                columns: 1,
                rows: 1,
                children: rsx! {
                    p {
                        span {
                            "I'm a first year Master's student in the Computer Science Department at Stanford University. "
                            "I'm currently working on embodied AI at the "
                        }
                        a { href: "https://svl.stanford.edu/", target: "_blank", "Stanford Vision & Learning Laboratory (SVL)" }
                        span { "." }
                    }
                    p {
                        span {
                            "Prior to Stanford, I spent a few years in industry working on safe reinforcement learning. "
                            "I received my B.S. in Computer Science and my B.A. in Criminology, Law, and Society from UCI, "
                            "where I had the pleasure of being mentored by "
                        }
                        a { href: "https://faculty.sites.uci.edu/wmmaurer/", target: "_blank", "Dr. Bill Maurer" }
                        span { " and "}
                        a { href: "https://www.tonykcheng.com/", target: "_blank", "Dr. Tony Cheng" }
                        span { "." }

                    }
                    ul { class: "contact-links",
                        li {
                            a {
                                class: "media-icon",
                                href: "mailto:aditesh@stanford.edu",
                                style: "font-size: 1.25rem; line-height: 1.25rem; margin-top: 0.5rem;",
                                span { class: "fa fa-envelope" }
                            }
                        }
                        li {
                            a {
                                class: "media-icon",
                                href: "https://scholar.google.com/citations?user=QayDh0IAAAAJ",
                                span { class: "ai ai-google-scholar" }
                            }
                        }
                        li {
                            a {
                                class: "media-icon",
                                href: "https://www.semanticscholar.org/author/Aditesh-Kumar/2219660996",
                                span { class: "ai ai-semantic-scholar" }
                            }
                        }
                        li {
                            a {
                                class: "media-icon",
                                href: "https://www.github.com/aramuk",
                                span { class: "fa fa-github" }
                            }
                        }
                    }
                }
            }
            // div {
            //     style: "color: var(--gray)",
            //     "scroll_y was {scroll_y}; event_text is {event_text}"
            // }
            Section {
                id: "pubs",
                title: "Publications",
                columns: 1,
                rows: 1,
                children: rsx! {
                    Publication {
                        title: "MMSum: A Dataset for Multimodal Summarization and Thumbnail Generation of Videos",
                        authors: vec![
                            "Jielin Qiu".to_string(),
                            "Jiacheng Zhu".to_string(),
                            "William Han".to_string(),
                            "Aditesh Kumar".to_string(),
                            "Karthik Mittal".to_string(),
                            "Claire Jin".to_string(),
                            "Zhengyuan Yang".to_string(),
                            "Linjie Li".to_string(),
                            "Jianfeng Wang".to_string(),
                            "Ding Zhao".to_string(),
                            "Bo Li".to_string(),
                            "Lijuan Wang".to_string()
                        ],
                        link: "https://cvpr.thecvf.com/virtual/2024/poster/31649",
                        venue: "IEEE/CVF Conference on Computer Vision and Pattern Recognition (CVPR). 2024.",
                        notes: "Highlight."
                    }
                }
            }
        }
        footer {
            {"Copyright 2020-2025 Aditesh Kumar"}
        }
    }
}
