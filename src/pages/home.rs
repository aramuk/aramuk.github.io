use dioxus::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Event};

use crate::components::header::*;
use crate::components::hero::*;
use crate::components::publication::*;
use crate::components::section::*;

const HEADER_BACKGROUND_RATIO: f64 = 0.1;

// Home page
#[component]
pub fn Home() -> Element {
    let mut scroll_y = use_signal(|| 0.);
    
    let onscroll = Closure::wrap(Box::new(move |e: Event| {
        web_sys::console::log_2(&"Scroll y:".into(), &window().unwrap().scroll_y().unwrap().into());
        scroll_y.set(window().unwrap().scroll_y().unwrap());
    }) as Box<dyn FnMut(Event)>);

    window()
        .unwrap()
        .set_onscroll(Some(onscroll.as_ref().unchecked_ref()));
    onscroll.forget();

    rsx! {
        body {
            Header { scrolled: scroll_y() > HEADER_BACKGROUND_RATIO * window().unwrap().inner_height().unwrap().as_f64().unwrap() }
            main {
                id: "main",
                Hero {
                    title: "Currently:",
                    subtitles: vec![
                        "MSCS @ Stanford".to_string(),
                        "Research Assistant @ SVL".to_string(),
                        "Course Assistant @ CS 231N".to_string()
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
                                "My current research focus is on embodied AI at the "
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
                                    aria_label: "Send an email to aditesh@stanford.edu",
                                    href: "mailto:aditesh@stanford.edu",
                                    style: "font-size: 1.25rem; line-height: 1.25rem; margin-top: 0.5rem;",
                                    i { aria_hidden: true, class: "fa fa-envelope" }
                                }
                            }
                            li {
                                a {
                                    class: "media-icon",
                                    aria_label: "Open Aditesh's Google Scholar page.",
                                    href: "https://scholar.google.com/citations?user=QayDh0IAAAAJ",
                                    i { aria_hidden: true, class: "ai ai-google-scholar" }
                                }
                            }
                            li {
                                a {
                                    class: "media-icon",
                                    aria_label: "Open Aditesh's Semantic Scholar page.",
                                    href: "https://www.semanticscholar.org/author/Aditesh-Kumar/2219660996",
                                    i { aria_hidden: true, class: "ai ai-semantic-scholar" }
                                }
                            }
                            li {
                                a {
                                    class: "media-icon",
                                    aria_label: "Open Aditesh's Github.",
                                    href: "https://www.github.com/aramuk",
                                    i { aria_hidden: true, class: "fa fa-github" }
                                }
                            }
                        }
                    }
                }
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
}
