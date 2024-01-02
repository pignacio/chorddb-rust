use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct SongHeader {
    id: Uuid,
    author: String,
    title: String,
}

impl SongHeader {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn author(&self) -> &str {
        self.author.as_ref()
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Song {
    header: SongHeader,
    contents: String,
}

impl Song {
    pub fn new(id: Uuid, author: String, title: String, contents: String) -> Self {
        Song{ header: SongHeader{id, author, title}, contents }
    }

    pub fn id(&self) -> &Uuid {
        return &self.header.id();
    }

    pub fn author(&self) -> &str {
        return &self.header.author();
    }

    pub fn title(&self) -> &str {
        return &self.header.title();
    }

    pub fn header(&self) -> &SongHeader {
        return &self.header;
    }

    pub fn contents(&self) -> &str {
        return &self.contents;
    }
}

pub trait SongRepository {
    fn all_songs(&self) -> Vec<SongHeader>;
    fn add_song(&self, song: Song);
    fn get_song(&self, id: &Uuid) -> Option<Song>;
}

pub struct MemorySongs {
    songs: RwLock<HashMap<Uuid, Song>>,
}

impl MemorySongs {
    pub fn new() -> Self {
        return MemorySongs {
            songs: HashMap::new().into(),
        };
    }

    fn read_songs(&self) -> RwLockReadGuard<'_, HashMap<Uuid, Song>> {
        return self.songs.read().unwrap();
    }

    fn write_songs(&self) -> RwLockWriteGuard<'_, HashMap<Uuid, Song>> {
        return self.songs.write().unwrap();
    }
}

impl SongRepository for MemorySongs {
    fn all_songs(&self) -> Vec<SongHeader> {
        let songs = self.read_songs();
        songs
            .values()
            .map(|song| song.header())
            .cloned()
            .collect()
    }

    fn get_song(&self, id: &Uuid) -> Option<Song> {
        let songs = self.read_songs();
        return songs.get(id).cloned();
    }

    fn add_song(&self, song: Song) {
        let mut songs = self.write_songs();
        songs.insert(song.id().clone(), song);
    }
}

pub struct FileSongs {
    path: String,
    cache: MemorySongs,
}

fn load_cache(path: &str) -> MemorySongs {
    let data: Vec<Song> = std::fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or(Vec::new());

    let songs = MemorySongs::new();

    for song in data {
        songs.add_song(song)
    }

    return songs;
}

impl FileSongs {
    pub fn new(path: &str) -> Self {
        FileSongs {
            path: path.to_owned(),
            cache: load_cache(path),
        }
    }

    fn save_cache(&self) {
        let songs: Vec<Song> = self
            .cache
            .all_songs()
            .iter()
            .filter_map(|header| self.cache.get_song(&header.id))
            .collect();
        std::fs::write(&self.path, serde_json::to_string(&songs).unwrap()).unwrap();
    }
}

impl SongRepository for FileSongs {
    fn all_songs(&self) -> Vec<SongHeader> {
        self.cache.all_songs()
    }

    fn add_song(&self, song: Song) {
        self.cache.add_song(song);
        self.save_cache();
    }

    fn get_song(&self, id: &Uuid) -> Option<Song> {
        self.cache.get_song(id)
    }
}
