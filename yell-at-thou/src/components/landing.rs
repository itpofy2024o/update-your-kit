use yew::prelude::*;
use gloo::timers::callback::Timeout;

#[function_component]
pub fn Landing() -> Html {
    let fade_in = use_state(|| false);

    {
        let fade_in = fade_in.clone();
        Timeout::new(1000, move || fade_in.set(true)).forget();
    }

    let class_name = if *fade_in { "fade-in" } else { "fade-out" };

    html! {
        <div class={class_name}>
            <h1>{ "Welcome to our Landing Page" }</h1>
        </div>
    }
}
