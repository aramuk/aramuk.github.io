use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;
use yew::web_sys::window;

use crate::components::Header;
use crate::components::Banner;

// #[derive(Properties, Clone, PartialEq)]
// pub struct Props {
//     #[prop_or(Anchor::Top)]
//     pub to: Anchor,
// }

pub struct HomePage {
    link: ComponentLink<Self>,
    // props: Props,
    scrolled: bool,
}

pub enum Msg {
    Scroll(f64),
}

// #[derive(Debug)]
// #[derive(Clone, PartialEq)]
// pub enum Anchor {
//     Top,
//     About,
//     Projects,
// }

// impl Default for Anchor {
//     fn default() -> Self {
//         Anchor::Top
//     }
// }

impl Component for HomePage {
    type Message = Msg;
    // type Properties = Props;
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

        Self { 
            link, 
            // props,
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // if self.props != props {
        //     console::log_1(&JsValue::from_str(&format!("Received props")));
        //     if self.props.to != props.to {
        //         let window = window().unwrap();
        //         console::log_1(&JsValue::from_str(&format!("Received props.to {:?}", props.to.clone())));
        //         console::log_1(&JsValue::from_str(&format!("Scrolling to {}", HomePage::get_anchor_position(&props.to) as f64)));
        //         window.scroll_to_with_x_and_y(0.0, HomePage::get_anchor_position(&props.to) as f64);
        //     }
        //     self.props = props;
        // }
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
                    <div id="about"><p>{"Who am I?"}</p></div>
                    <div id="projects"><p>{"What have I done?"}</p></div>
                </main>
            </div>
        }
    }
}

// impl HomePage {
//     fn get_anchor_str(anchor: &Anchor) -> Option<&str> {
//         match anchor {
//             Anchor::About => Some("about"),
//             Anchor::Projects => Some("projects"),
//             _ => None,
//         }
//     }

//     fn get_anchor_position(anchor: &Anchor) -> i32 {
//         let window = window().expect("`window` not present");
//         let document = window.document().expect("`document` not present");
//         if let Some(anchor_str) = HomePage::get_anchor_str(anchor) {
//             let elem = document.get_element_by_id(&anchor_str)
//                 .expect(&format!("There is no anchor named `{}`", anchor_str));
//             console::log_1(&JsValue::from_str(&format!("Going to about at: {}", elem.client_top())));
//             elem.client_top()
//         } else {
//             0
//         }
//     }
// }