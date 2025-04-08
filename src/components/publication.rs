use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PublicationProps {
    title: String,
    authors: Vec<String>,
    link: String,
    venue: String,
    notes: String,
}

#[component]
pub fn Publication(props: PublicationProps) -> Element {
    rsx! {
        div { class: "project",
            div { class: "project-title",
                a {
                    href: props.link,
                    target: "_blank",
                    h3 { "{props.title}" }
                }
            }
            div { class: "project-desc",
                p {
                    {props.authors.into_iter().enumerate().map(|(i, author)| {
                        rsx! {
                            {if i > 0 { rsx! { span {", "}}} else { rsx!{}}}
                            {if author == "Aditesh Kumar" {
                                rsx!{ b { "{author}" } }
                            } else {
                                rsx!{ span { "{author}" }}
                            }}
                        }
                    })}
                }
                p { i { "{props.venue}" } }
                p { "{props.notes}" }
            }
        }
    }
}
