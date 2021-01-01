use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;
use yew::web_sys::window;

use crate::components::Footer;
use crate::components::Header;
use crate::components::Banner;
use crate::components::Project;
use crate::components::Section;

pub struct HomePage {
    link: ComponentLink<Self>,
    scrolled: bool,
}

pub enum Msg {
    Scroll(f64),
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_: Event| {
            Msg::Scroll(window().unwrap().scroll_y().unwrap())
        });
        let onscroll = Closure::wrap(Box::new(move |e: Event| {
            callback.emit(e);
        }) as Box<dyn FnMut(Event)>);

        window().unwrap().set_onscroll(Some(onscroll.as_ref().unchecked_ref()));
        onscroll.forget();

        HomePage { 
            link,
            scrolled: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Scroll(y) => {
                if !self.scrolled && y > 0.0 {
                    self.scrolled = true;
                    true
                } else if self.scrolled && y <= 0.0{
                    self.scrolled = false;
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Header background={self.scrolled} />
                <main>
                    <Banner
                        title="${JOB_TITLE}"
                        subtitle="${EMPLOYER}"
                        image="./images/lake_tahoe.jpeg" 
                    />
                    <Section 
                        id="about"
                        title="Who am I?"
                        columns={1}
                        rows={2}
                    >
                        <p>
                            {"I'd prefer to have Google work a little bit harder to know that, \
                            but you can find out here:"}
                        </p>
                        <div class="contact-links">
                            <div>
                                <a href="https://www.github.com/aramuk" class="media-icon">
                                    <span class="fa fa-github"></span>
                                </a>
                            </div>
                            <div>
                                <a href="https://www.linkedin.com/in/aditeshk" class="media-icon">
                                    <span class="fa fa-linkedin-square"></span>
                                </a>
                            </div>
                            <div>
                                <a href="mailto:aditeshk@uci.edu" class="media-icon">
                                    <span class="fa fa-envelope"></span>
                                </a>
                            </div>
                        </div>
                    </Section>
                    <Section 
                        id="projects"
                        title="Projects"
                        columns={2}
                        rows={2}
                    >
                        <Project 
                            title="Histopathology"
                            github_link="https://www.github.com/aramuk/histopathology"
                            description="Convolutional neural networks that detect cancer in \
                                histopathology slides a bit worse than a doctor can, but at least \
                                they do it free."
                        />
                        <Project 
                            title="Personal Website"
                            github_link="https://www.github.com/aramuk/aramuk.github.io"
                            description="My personal website, written in Rust, because WASM is cool \
                                and any website that's easy to maintain doesn't truly capture the \
                                spirit of web dev (See also: recursion)."
                        />
                        <Project 
                            title="WalkWithMe"
                            github_link="https://www.github.com/aramuk/WalkWithMe"
                            description="Uber Pool, but for pedestrians. A mobile app that helps people \
                                find friends to walk around with."
                        />
                        <Project 
                            title="TNG"
                            github_link="https://www.github.com/aramuk/tng"
                            description="A library for the retrieval, processing, and management, of 
                            image data from the Illustris simulation. No relation to that show \
                            starring Sir Patrick Stewart."
                        />
                    </Section>
                </main>
                <Footer />
            </div>
        }
    }
}
