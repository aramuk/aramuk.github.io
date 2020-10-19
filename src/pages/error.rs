use yew::prelude::*;
use yew::Properties;

pub struct ErrorPage {
    link: ComponentLink<Self>,
    error: Option<String>,
}

pub struct Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub error: Option<String>,
}

impl Component for ErrorPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { 
            link,
            error: props.error,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
            <div>
                <h1>{"Error Page"}</h1>
                {self.display_error()}
            </div>
        }
    }
}

impl ErrorPage {
    fn display_error(&self) -> Html {
        if let Some(error) = &self.error {
            return html! { <p><b>{error}</b></p> }
        }
        return html! {}
    }
}