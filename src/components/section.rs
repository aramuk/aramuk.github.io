use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SectionProps {
    pub id: String,
    pub title: String,
    pub columns: i32,
    pub rows: i32,
    pub children: Children,
}

pub struct Section {
    props: SectionProps,
}

impl Component for Section {
    type Message = ();
    type Properties = SectionProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Section {
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
            <div id={self.props.id.clone()} class="section">
                <div class="section-title">
                    <h2>{self.props.title.clone()}</h2>
                </div>
                <div 
                    class="section-grid"
                    style={format!("grid-template-columns: repeat({}, auto); grid-template-rows: repeat({}, auto);", self.props.columns, self.props.rows)}
                >
                    {for self.props.children.iter()}
                </div>
            </div>
        }
    }
}