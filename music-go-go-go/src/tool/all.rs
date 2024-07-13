// src/read_all.rs

use yew::prelude::*;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use crate::models::Item;

pub struct ReadAll {
    fetch_task: Option<FetchTask>,
    items: Vec<Item>,
}

pub enum Msg {
    FetchItems,
    FetchItemsSuccess(Vec<Item>),
    FetchItemsError,
}

impl Component for ReadAll {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::FetchItems);
        ReadAll {
            fetch_task: None,
            items: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchItems => {
                let request = Request::get("/api/items")
                    .body(Nothing)
                    .expect("Failed to build request.");

                let fetch_task = FetchService::fetch(
                    request,
                    link.callback(|response: Response<Json<Result<Vec<Item>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(items) => Msg::FetchItemsSuccess(items),
                            Err(_) => Msg::FetchItemsError,
                        }
                    }),
                )
                .expect("Failed to start request.");

                self.fetch_task = Some(fetch_task);
                true
            }
            Msg::FetchItemsSuccess(items) => {
                self.items = items;
                true
            }
            Msg::FetchItemsError => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{"Read All Items"}</h2>
                <ul>
                    { for self.items.iter().map(|item| self.view_item(item)) }
                </ul>
            </div>
        }
    }
}

impl ReadAll {
    fn view_item(&self, item: &Item) -> Html {
        html! {
            <li>{ &item.name }</li>
        }
    }
}
