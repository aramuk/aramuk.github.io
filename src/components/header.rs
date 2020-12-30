use yew::prelude::*;

use crate::components::NavLink;

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    pub background: bool,
}

pub struct Header {
    props: HeaderProps,
}

impl Component for Header {
    type Message = ();
    type Properties = HeaderProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Header {
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
            <header>
                <nav class={if self.props.background { "navigation bg" } else { "navigation" } }>
                    <ul>
                        <NavLink link_text="Aditesh Kumar" section_id="/" background={self.props.background} title={true} />
                        <NavLink link_text="About" section_id="#about" background={self.props.background} title={false} />
                        <NavLink link_text="Projects" section_id="#projects" background={self.props.background} title={false} />
                    </ul>
                </nav>
            </header>
        }
    }
}
