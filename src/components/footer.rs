use yew::prelude::*;

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn change(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn update(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {
            <footer>
                {"Copyright 2020 Aditesh Kumar."}
            </footer>
        }
    }
}