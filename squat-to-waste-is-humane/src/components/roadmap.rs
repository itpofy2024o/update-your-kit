use yew::prelude::*;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <div class="landing">
            <h1>{ "Welcome to My Website" }</h1>
            <p>{ "This is the landing page." }</p>
        </div>
    }
}
