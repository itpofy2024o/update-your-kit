use yew::prelude::*;
use yew::services::timeout::{TimeoutService, TimeoutTask};
use gloo::storage::{Storage, LocalStorage};
use gloo::storage::errors::StorageError;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

const ANIMATION_DURATION: i32 = 3500;

#[function_component(Animation)]
fn animation() -> Html {
    let node_ref = use_node_ref();
    let animation_done = use_state(|| false);

    {
        let animation_done = animation_done.clone();
        use_effect_with_deps(
            move |_| {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let body = document.body().unwrap();
                let cat_element = node_ref.cast::<HtmlElement>().unwrap();
                body.append_child(&cat_element).unwrap();

                let keyframes = js_sys::Array::new();
                keyframes.push(&js_sys::Object::from(serde_wasm_bindgen::to_value(&serde_json::json!({
                    "transform": "translateX(100vw)"
                })).unwrap()));
                keyframes.push(&js_sys::Object::from(serde_wasm_bindgen::to_value(&serde_json::json!({
                    "transform": "translateX(-100vw)"
                })).unwrap()));

                let options = js_sys::Object::new();
                js_sys::Reflect::set(&options, &JsValue::from_str("duration"), &JsValue::from_f64(ANIMATION_DURATION as f64)).unwrap();
                js_sys::Reflect::set(&options, &JsValue::from_str("iterations"), &JsValue::from_f64(1.0)).unwrap();

                let animation = cat_element.animate_with_keyframes_and_options(&keyframes, &options);
                let animation_done_clone = animation_done.clone();
                let on_finish_closure = Closure::wrap(Box::new(move || {
                    animation_done_clone.set(true);
                }) as Box<dyn FnMut()>);
                animation.set_onfinish(Some(on_finish_closure.as_ref().unchecked_ref()));
                on_finish_closure.forget();

                || ()
            },
            (),
        );
    }

    let svg_content = include_str!("../zyx.svg");

    html! {
        <div>
            { yew::virtual_dom::VNode::VRef(svg_content.into()) }
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let has_animated = {
        LocalStorage::get("has_animated").unwrap_or(false)
    };

    if !has_animated {
        LocalStorage::set("has_animated", true).unwrap();
    }

    html! {
        <>
            { if !has_animated {
                html! { <Animation /> }
            } else {
                html! { <div>{ "Home Page Content" }</div> }
            }}
        </>
    }
}
