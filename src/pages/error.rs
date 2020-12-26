use yew::prelude::*;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub error: Option<String>,
}

pub struct ErrorPage {
    link: ComponentLink<Self>,
    props: Props,
}

pub struct Msg {}

impl Component for ErrorPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
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
                <h1>{"Error Page"}</h1>
                {if let Some(error) = &self.props.error {
                    html! { <p><b>{error}</b></p> }
                } else { html!{} }}
            </main>
        }
    }
}
