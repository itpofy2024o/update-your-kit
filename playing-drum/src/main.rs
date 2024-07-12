use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Music {
    id: u32,
    name: String,
    author: String,
}

fn app() -> Template<()> {
    let users = create_signal(vec![]);

    {
        let users = users.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_users: Vec<User> = Request::get("http://localhost:8080/api/users")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            users.set(fetched_users);
        })
    }

    template! {
        div {
            h1 { "User List" }
            ul {
                Indexed {
                    iterable: users,
                    view: |user| template! {
                        li { (format!("{} - {}", user.name, user.email)) }
                    }
                }
            }
        }
    }
}

fn main() {
    sycamore::render_to_static_markup(|| App {});
}
