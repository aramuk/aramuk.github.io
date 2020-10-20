use yew::prelude::*;

use crate::components::Header;

pub struct HomePage {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Scroll,
}

impl Component for HomePage {
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
            <main>
                <Header />    
            </main>
        }
    }
}
