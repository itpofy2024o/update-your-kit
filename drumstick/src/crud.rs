use crate::models::Music;
use std::collections::HashMap;

pub struct RhythmStore {
    music: HashMap<u32, Music>,
    next_id: u32,
}

impl RhythmStore {
    pub fn new() -> Self {
        RhythmStore {
            music: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create(&mut self, name: String, author: String) -> Music {
        let music = Music {
            id: self.next_id,
            name,
            author,
        };
        self.music.insert(self.next_id, music.clone());
        self.next_id += 1;
        music
    }

    // pub fn read(&self, id: u32) -> Option<&Music> {
    //     self.music.get(&id)
    // }

    pub fn update(&mut self, id: u32, name: String, author: String) -> Option<Music> {
        if let Some(music) = self.music.get_mut(&id) {
            music.name = name.clone();
            music.author = author.clone();
            return Some(music.clone());
        }
        None
    }

    pub fn delete(&mut self, id: u32) -> bool {
        self.music.remove(&id).is_some()
    }

    pub fn list(&self) -> Vec<Music> {
        self.music.values().cloned().collect()
    }
}
