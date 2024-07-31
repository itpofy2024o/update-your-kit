use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew_assets::Assets;

const ANIMATION_DURATION: i32 = 3500; 

#[function_component]
pub fn Animation() -> Html {
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

    html! {
        <div>
            <img ref={node_ref} src="/zyx.svg" alt="cat animation" />
        </div>
    }
}
