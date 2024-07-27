use yew::prelude::*;
use crate::components::goods::Goods;

#[function_component(GoodsPage)]
pub fn goods_page() -> Html {
    html! {
        <Goods />
    }
}
