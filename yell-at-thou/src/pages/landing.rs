use crate::components::animation::Animation;
use crate::components::landing::Landing;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

#[function_component]
pub fn LandingPage() -> Html {
    let has_animated = { LocalStorage::get("has_animated").unwrap_or(false) };

    if !has_animated {
        LocalStorage::set("has_animated", true).unwrap();
    }

    // html! {
    //     <>
    //         { if !has_animated {
    //             html! { <Animation /> }
    //         } else {
    //             html! {
    //                 <Landing /> }
    //         }}
    //     </>
    // }
    html! {
        <>
                    <Landing />
        </>
    }
}
