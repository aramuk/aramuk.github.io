use yew::prelude::*;

pub struct NavLink {
    link: ComponentLink<Self>,
    link_text: String,
    section_id: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub link_text: String,
    pub section_id: String,
}

pub enum Msg {
    Clicked,
}

impl Component for NavLink {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            link_text: props.link_text,
            section_id: props.section_id,
        }
    }

    fn update(&mut self, _props: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <li class="section-link">
                <a href={self.section_id.clone()}>
                    {self.link_text.clone()}
                </a>
            </li>
        }
    }
}
