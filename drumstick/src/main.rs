mod crud;
mod models;
mod api;

use std::sync::{Arc, Mutex};

use crate::crud::RhythmStore;

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(RhythmStore::new()));
    let routes = api::routes(store);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
