use dioxus::prelude::*;
mod splash;
mod crud;

#[derive(PartialEq, Props)]
struct AppState {
    is_loading: UseState<bool>,
}

pub fn app(cx: Scope) -> Element {
    let is_loading = use_state(&cx, || true);

    cx.render(rsx!(
        if *is_loading.get() {
            rsx!(splash::Splash { is_loading: is_loading.clone() })
        } else {
            rsx!(crud::CrudApp {})
        }
    ))
}
