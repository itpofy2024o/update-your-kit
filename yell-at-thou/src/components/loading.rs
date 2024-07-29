use yew::prelude::*;
use yew::services::{TimeoutService, timeout::TimeoutTask};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use std::time::Duration;

struct Model {
    link: ComponentLink<Self>,
    animation_task: Option<TimeoutTask>,
}

enum Msg {
    Animate,
    AnimationComplete,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        link.send_message(Msg::Animate);

        Self {
            link,
            animation_task: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Animate => {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let dog_element = document.get_element_by_id("dog").unwrap().dyn_into::<HtmlElement>().unwrap();

                dog_element.set_class_name("animate");

                let callback = ctx.link().callback(|_| Msg::AnimationComplete);
                let handle = TimeoutService::spawn(Duration::from_secs(5), callback);
                self.animation_task = Some(handle);

                true
            }
            Msg::AnimationComplete => {
                // Handle animation complete logic if necessary
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <img id="dog" src="/static/dog.svg" alt="Dog" />
            </div>
        }
    }
}