use dioxus::prelude::*;
use std::time::Duration;

#[derive(PartialEq, Props)]
pub struct SplashProps {
    pub is_loading: UseState<bool>,
}

pub fn Splash(cx: Scope<SplashProps>) -> Element {
    use_effect(cx, (), {
        let is_loading = cx.props.is_loading.clone();
        async move {
            // Simulate loading
            tokio::time::sleep(Duration::from_secs(3)).await;
            is_loading.set(false);
        }
    });

    cx.render(rsx!(
        div {
            class: "splash-screen",
            style: "display: flex; justify-content: center; align-items: center; height: 100vh;",
            img {
                src: "static/zyx.svg",
                alt: "Loading",
                style: "width: 100px; height: 100px;",
            }
        }
    ))
}
