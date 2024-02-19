use axum::{
    extract::{Path, State},
    Json,
};

use crate::chord::{
    finder::{Fingering, StringInstrument, GUITAR_STANDARD},
    Chord,
};

use super::AppState;

pub async fn chords(
    Path((instrument, chord)): Path<(String, String)>,
    State(AppState { chords, .. }): State<AppState>,
) -> Json<Vec<String>> {
    let Some(chord) = Chord::parse(chord) else {
        return Json(vec![]);
    };
    let Some(instrument) = parse_instrument(instrument) else {
        return Json(vec![]);
    };
    let response = chords
        .get_fingerings(instrument, &chord)
        .iter()
        .map(Fingering::to_str)
        .collect();
    Json::<Vec<_>>(response)
}

fn parse_instrument(_instrument: String) -> Option<&'static StringInstrument> {
    Some(&GUITAR_STANDARD)
}
