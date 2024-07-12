use crate::crud::RhythmStore;
use crate::models::Music;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply, http::StatusCode};

#[derive(Debug, Deserialize, Clone)]
struct InputSong {
    name: String,
    author: String,
}

pub fn routes(
    store: Arc<Mutex<RhythmStore>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let store = warp::any().map(move || store.clone());

    let list = warp::path!("allmusic")
        .and(warp::get())
        .and(store.clone())
        .and_then(list_all_music);

    let create = warp::path!("addmusic")
        .and(warp::post())
        .and(warp::body::json())
        .and(store.clone())
        .and_then(create_music);

    // let update = warp::path!("oldmusic" / u32)
    //     .and(warp::put())
    //     .and(warp::body::json())
    //     .and(store.clone())
    //     .and_then(update_song);

    // let delete = warp::path!("binmusic" / u32)
    //     .and(warp::delete())
    //     .and(store.clone())
    //     .and_then(delete_instance);

    list.or(create)
}

async fn list_all_music(store: Arc<Mutex<RhythmStore>>) -> Result<impl warp::Reply, warp::Rejection> {
    let store = store.lock().unwrap();
    let music = store.list();
    Ok(warp::reply::json(&music))
}

async fn create_music(
    music: InputSong,
    store: Arc<Mutex<RhythmStore>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut store = store.lock().unwrap();
    let input = store.create(music.name, music.author);
    Ok(warp::reply::with_status(warp::reply::json(&input), StatusCode::CREATED))
}

// async fn update_song(
//     id: u32,
//     music: InputSong,
//     store: Arc<Mutex<RhythmStore>>,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     let mut store = store.lock().unwrap();
//     match store.update(id, music.name, music.author) {
//         Some(music) => Ok(warp::reply::json(&music)),
//         None => Ok(warp::reply::json({"error": "Music not found"})),
//     }
// }

// async fn delete_instance(
//     id: u32,
//     store: Arc<Mutex<RhythmStore>>,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     let mut store = store.lock().unwrap();
//     if store.delete(id) {
//         Ok(warp::reply::json(&json!({"status": "Deleted"})))
//     } else {
//         Ok(warp::reply::json(&json!({"error": "not found"})))
//     }
// }
