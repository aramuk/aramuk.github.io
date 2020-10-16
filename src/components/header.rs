use yew::prelude::*;

pub struct Header {
    link: ComponentLink<Self>
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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <header>
                <div class="name">
                    <h1>{"Aditesh Kumar"}</h1>
                </div>
                <div class="navigation">
                    <div class="section-link">{"Projects"}</div>
                    <div class="section-link">{"Experience"}</div>
                    <div class="section-link">{"Links"}</div>
                </div>
            </header>
        }
    }
}