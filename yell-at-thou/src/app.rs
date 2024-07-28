use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::landing::LandingPage;

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/")]
    Landing,
    // #[at("/products")]
    // Goods,
}

fn switches(routes: Route) -> Html {
    match routes {
        Route::Landing => html! { <LandingPage /> },
        // Route::Goods => html! { <GoodsPage /> },
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
