use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SectionProps {
    id: String,
    title: String,
    columns: i32,
    rows: i32,
    children: Element,
}

pub fn Section(props: SectionProps) -> Element {
    rsx! {
        div { class: "section", id: props.id,
            div { class: "section-title",
                h2 { "{props.title}" }
            }
            div {
                class: "section-grid",
                style: format!("grid-template-columns: repeat({}, 1fr); grid-template-rows: repeat({}, 1fr);", props.columns, props.rows),
                {props.children.into_iter()}
            }
        }
    }
}
