use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::landing::LandingPage;

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/")]
    Landing,
    // #[at("/products")]
    // Goods,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switches(routes: Route) -> Html {
    match routes {
        Route::Landing => html! { <LandingPage /> },
        // Route::Goods => html! { <GoodsPage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switches} />
        </BrowserRouter>
    }
}
