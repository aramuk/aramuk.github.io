use yew::prelude::*;
use crate::components::NavLink;

pub struct Header {
    link: ComponentLink<Self>,
}

pub enum Msg {
    StickHeader,
    ReleaseHeader,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <header>
                <div class="title">
                    <h1>{"Aditesh Kumar"}</h1>
                </div>
                <nav class="navigation">
                    <ul>
                        <NavLink link_text="Projects" section_id="#projects" />
                        <NavLink link_text="Experience" section_id="#experience" />
                        <NavLink link_text="Contact" section_id="#contact" />
                    </ul>
                </nav>
                <div class="triangle"></div>
            </header>
        }
    }
}
