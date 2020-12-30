use yew::prelude::*;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct ErrorPageProps {
    #[prop_or_default]
    pub error: Option<String>,
}

pub struct ErrorPage {
    props: ErrorPageProps,
}

impl Component for ErrorPage {
    type Message = ();
    type Properties = ErrorPageProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ErrorPage { 
            props 
        }
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
