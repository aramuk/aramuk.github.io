use yew::prelude::*;
use yew_router::components::RouterAnchor;

#[derive(Properties, Clone, PartialEq)]
pub struct NavLinkProps {
    pub link_text: String,
    pub section_id: String,
    #[prop_or(true)]
    pub background: bool,
    #[prop_or(false)]
    pub title: bool,
}

pub struct NavLink {
    props: NavLinkProps,
}

impl Component for NavLink {
    type Message = ();
    type Properties = NavLinkProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NavLink {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <li class={ if self.props.background { "section-link bg" } else { "section-link"} }>
                {if self.props.title {
                    html! {
                        <RouterAnchor<String>
                            route={self.props.section_id.clone()}
                            classes="title"
                        >
                            <h1>
                                {self.props.link_text.clone()}
                            </h1>
                        </RouterAnchor<String>>
                    }
                } else {
                    html! {
                        <a href={self.props.section_id.clone()}>
                            <h3>{self.props.link_text.clone()}</h3>
                        </a>
                    }
                }}
            </li>
        }
    }
}
