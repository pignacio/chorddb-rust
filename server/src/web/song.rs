use std::collections::HashMap;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    chord::finder::GUITAR_STANDARD,
    parser::{parse_tablature, Line, LineBit},
    song::{ChordRepository, Song},
    web::not_found,
};

use super::AppState;

#[derive(Debug, PartialEq, Eq, Serialize)]
struct ChordModel {
    chord: String,
    fingering: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct LineBitModel {
    #[serde(rename = "type")]
    bit_type: String,
    position: usize,
    text: String,
    chord: Option<ChordModel>,
}

#[derive(Deserialize)]
pub struct AddSong {
    author: String,
    title: String,
    contents: String,
}

pub async fn add_song(
    State(AppState { songs, .. }): State<AppState>,
    Form(payload): Form<AddSong>,
) -> Redirect {
    let song = Song::new(
        Uuid::new_v4(),
        payload.author,
        payload.title,
        payload.contents,
    );

    let id = song.id().clone();
    songs.add_song(song);

    return Redirect::to(&format!("/songs/{}", id));
}

#[derive(Template)]
#[template(path = "song.html")]
struct SongTemplate {
    song: Song,
    tab: String,
}

pub async fn song(
    Path(id): Path<String>,
    State(AppState { songs, chords, .. }): State<AppState>,
) -> impl IntoResponse {
    return Html(
        Uuid::parse_str(&id)
            .ok()
            .and_then(|song_id| songs.get_song(&song_id))
            .map(|song| {
                SongTemplate {
                    song: song.clone(),
                    tab: test(&song, chords.as_ref()),
                }
                .render()
                .unwrap()
            })
            .unwrap_or_else(|| {
                println!("Could not find song for '{}'", &id);
                return not_found::not_found_html();
            }),
    );
}

fn test(song: &Song, chords: &dyn ChordRepository) -> String {
    let tab = parse_tablature(song.contents());
    let model: Vec<Vec<LineBitModel>> = tab.iter().map(|l| serialize_line(l, chords)).collect();
    serde_json::to_string(&model).unwrap()
}

fn serialize_line(line: &Line, chords: &dyn ChordRepository) -> Vec<LineBitModel> {
    line.iter().map(|b| serialize_bit(b, chords)).collect()
}

lazy_static! {
    static ref FINGERING_BY_CHORD: HashMap<String, String> = [
        ("A", "002220"),
        ("Bb", "x13331"),
        ("B", "x24442"),
        ("C", "032010"),
        ("Db", "x466644"),
        ("D", "xx0232"),
        ("Eb", "xx1343"),
        ("E", "022100"),
        ("F", "133211"),
        ("Gb", "244322"),
        ("G", "320003"),
        ("Ab", "466544"),
        ("A7", "002020"),
        ("Bb7", "x13131"),
        ("B7", "x21202"),
        ("C7", "x35353"),
        ("Db7", "x46464"),
        ("D7", "xx0212"),
        ("Eb7", "xx1323"),
        ("E7", "020130"),
        ("F7", "131211"),
        ("Gb7", "242322"),
        ("G7", "320001"),
        ("Ab7", "464544"),
        ("Am", "002210"),
        ("Bbm", "x13321"),
        ("Bm", "x24432"),
        ("Cm", "035543"),
        ("Dbm", "x466544"),
        ("Dm", "xx0231"),
        ("Ebm", "xx1342"),
        ("Em", "022000"),
        ("Fm", "133111"),
        ("Gbm", "244222"),
        ("Gm", "355333"),
        ("Abm", "466444"),
    ]
    .into_iter()
    .map(|(a, b)| (a.to_owned(), b.to_owned()))
    .collect();
}

fn serialize_bit(bit: &LineBit, chords: &dyn ChordRepository) -> LineBitModel {
    match &bit.comp {
        crate::parser::Comp::Text(text) => LineBitModel {
            bit_type: "text".to_owned(),
            position: bit.position,
            text: text.clone(),
            chord: None,
        },
        crate::parser::Comp::Chord {
            chord,
            original_text,
        } => LineBitModel {
            bit_type: "chord".to_owned(),
            position: bit.position,
            text: original_text.clone(),
            chord: Some(ChordModel {
                chord: chord.text(),
                fingering: chords
                    .get_fingerings(&GUITAR_STANDARD, chord)
                    .first()
                    .map(|f| f.to_str())
                    .unwrap_or("XXXXXX".to_owned()),
            }),
        },
    }
}
