use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ProjectProps {
    pub title: String,
    pub github_link: String,
    pub description: String,
}

pub struct Project {
    pub props: ProjectProps,
}

impl Component for Project {
    type Message = ();
    type Properties = ProjectProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Project {
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
            <div class="project">
                <div class="project-title">
                    <h3>{self.props.title.clone()}</h3>
                    <div class="project-gh">
                        <a href={self.props.github_link.clone()} class="media-icon">
                            <span class="fa fa-github"></span>
                        </a>
                    </div>
                </div>
                <div class="project-desc">
                    <p>{self.props.description.clone()}</p>
                </div>
            </div>
        }
    }
}