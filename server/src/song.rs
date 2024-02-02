use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::chord::{
    finder::{find_fingerings, Fingering, StringInstrument},
    Chord, ALL_KEYS, ALL_VARIANTS,
};

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
        Song {
            header: SongHeader { id, author, title },
            contents,
        }
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
        songs.values().map(|song| song.header()).cloned().collect()
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

pub trait ChordRepository {
    fn get_fingerings(&self, instrument: &'static StringInstrument, chord: &Chord) -> &[Fingering];
}

pub struct PrecomputedChords {
    instrument: &'static StringInstrument,
    fingerings: HashMap<Chord, Vec<Fingering>>,
}

impl PrecomputedChords {
    pub fn new(instrument: &'static StringInstrument) -> Self {
        let mut fingerings = HashMap::new();

        log::info!("Precomputing fingerings for all chords");
        for root in ALL_KEYS {
            log::info!("Precomputing all chords with root {:?}", root);
            for variant in ALL_VARIANTS {
                // for bass in ALL_KEYS {
                // let chord = Chord::new(root, variant, bass);
                let chord = Chord::new(root, variant, root);
                let mut chord_fingerings = find_fingerings(&chord, instrument);
                chord_fingerings.sort_by_cached_key(PrecomputedChords::fingering_penalty);
                let top = 10;
                log::info!(
                    "Top {} for {}: {}",
                    top,
                    chord,
                    chord_fingerings
                        .iter()
                        .take(top)
                        .map(|f| format!("{} ({})", f.to_str(), Self::fingering_penalty(f)))
                        .join(", ")
                );
                fingerings.insert(chord, chord_fingerings);
                // }
            }
        }

        log::info!("Done precomputing fingerings");

        PrecomputedChords {
            instrument,
            fingerings,
        }
    }

    fn fingering_score(fingering: &Fingering) -> usize {
        let mut score = fingering.placements().iter().map(|p| p.unwrap_or(2)).sum();
        if Self::has_note_hole(fingering) {
            score += 20;
        }
        score
    }

    fn has_note_hole(fingering: &Fingering) -> bool {
        let mut found_finger = false;
        let mut found_hole = false;
        for value in fingering.placements() {
            if let Some(_note) = value {
                if found_hole {
                    return true;
                }
                found_finger = true;
            } else {
                if found_finger {
                    found_hole = true;
                }
            }
        }
        false
    }

    fn has_bar_hole(fingering: &Fingering, bar: &usize) -> bool {
        let mut found_bar = false;
        for value in fingering.placements() {
            if let Some(note) = value {
                if note == bar {
                    found_bar = true;
                } else if 0 == *note && found_bar {
                    return true;
                }
            } else {
                if found_bar {
                    return true;
                }
            }
        }
        false
    }

    fn fingering_penalty(fingering: &Fingering) -> i32 {
        let mut bar = usize::MAX;
        let mut bar_count = 0;
        for placement in fingering.placements() {
            if let Some(value) = placement {
                if *value > 0 && bar > *value {
                    bar = *value;
                    bar_count = 1;
                } else if bar == *value {
                    bar_count += 1;
                }
            }
        }
        if bar == usize::MAX {
            bar = 0;
        }

        let mut finger_count = 0;
        for placement in fingering.placements() {
            if let Some(value) = placement {
                if *value > bar {
                    finger_count += 1;
                }
            }
        }

        if finger_count > 4 || (bar > 0 && finger_count > 3) {
            // Too many fingers!
            return 1000;
        }

        // Distance from the bar
        let mut score: i32 = fingering
            .placements()
            .iter()
            .filter(|x| matches!(x, Some(v) if *v > 0))
            .map(|x| x.map(|v| (v - bar) as i32).unwrap_or(0))
            .map(|x| x * x)
            .sum();

        // Favor chords lower on the neck
        score += bar as i32 * 4;

        // Does it skip strings at the start?
        let mut start = 0;
        for placement in fingering.placements() {
            if !matches!(placement, None) {
                break;
            }
            start += 1;
        }
        score += start * 10;

        // Does it skip strings at the end?
        let mut end = 0;
        for placement in fingering.placements().iter().rev() {
            if !matches!(placement, None) {
                break;
            }
            end += 1;
        }
        score += end * 10;

        // Does it have holes?
        if Self::has_note_hole(fingering) {
            score += 50;
        }
        if bar_count > 1 && bar_count + finger_count >= 4 && Self::has_bar_hole(fingering, &bar) {
            score += 50;
        }

        // Uses all the fingers
        if finger_count >= 3 {
            score += 10;
        }

        // Penalize big consecutive differences
        let mut last: Option<usize> = None;
        for placement in fingering.placements() {
            let Some(current) = placement else {
                continue;
            };
            if current > &0 {
                if let Some(last_value) = last {
                    let distance = last_value as i32 - *current as i32;
                    score += distance * distance;
                }
                last = Some(*current);
            }
        }

        score
    }

    /*
    def get_fingering_penalty(fingering):
    try:
        bar = min(x for x in fingering.positions if x)
    except ValueError:
        bar = 0

    indexed_poss = sorted(enumerate(x - bar for x in fingering.positions
                                    if x > bar),
                          key=lambda string_pos: (string_pos[1], string_pos[0]),
                          reverse=True)
    fingers = len(indexed_poss)
    if fingers > 4 or (bar and fingers > 3):
        return {'too many fingers': 10000}
    penalty = {
        'start': fingering.start * 5 ** 2,
        'end': (fingering.instrument.size() - fingering.start -
                len(fingering.positions)) * 8 ** 2,
        'positions': sum((p - bar + 2) ** 2 for p in fingering.positions),
        'bar': bar * fingering.instrument.size() * 3 if bar else 0,
        'consecutive_diffs': sum((a - b) ** 2 for a, b in
                                 zip(fingering.positions,
                                     fingering.positions[1:])
                                 if a and b),
        'four_fingers': 50 if fingers == 4 else 0
    }
    return get_fingering_penalty
    */
}

impl ChordRepository for PrecomputedChords {
    fn get_fingerings(&self, instrument: &'static StringInstrument, chord: &Chord) -> &[Fingering] {
        if instrument == self.instrument {
            self.fingerings.get(chord).map(|v| &v[..]).unwrap_or(&[])
        } else {
            &[]
        }
    }
}
