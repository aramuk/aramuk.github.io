use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct BannerProps {
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
    pub image: String,
}

pub struct Banner {
    props: BannerProps,
}

impl Component for Banner {
    type Message = ();
    type Properties = BannerProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Banner { 
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
            <div class="banner">
                <img src={self.props.image.clone()} />
                // <div class="banner-title">
                //     <h3>{self.props.title.clone()}</h3>
                //     {if let Some(subtitle) = &self.props.subtitle {
                //         html! {
                //             <h4>{subtitle}</h4>
                //         }
                //     } else { html!{} }}
                // </div>
            </div>
        }
    }
}
