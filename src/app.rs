use crate::pages::*;

use yew::prelude::*;
use yew_router::{prelude::*, switch::Permissive, Switch};

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/!"]
    Home,
    #[to = "/error"]
    NotFound(Permissive<String>),
}

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Error,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
            <Router<AppRoute, ()>
                render = Router::render(move |route: AppRoute| {
                    match route {
                        AppRoute::Home => html!{<HomePage />},
                        AppRoute::NotFound(Permissive(None)) => html!{<ErrorPage />},
                        AppRoute::NotFound(Permissive(Some(missed_route))) => html!{<ErrorPage error=missed_route />},
                        _ => html! {<ErrorPage />},
                    }
                })
                redirect = Router::redirect(|route: Route| {
                    AppRoute::NotFound(Permissive(Some(route.route)))
                })
            />
        }
    }
}