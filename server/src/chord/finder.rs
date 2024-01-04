use std::alloc::System;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::time::SystemTime;

use itertools::Itertools;

use super::Chord;
use super::Key;
use super::Note;

pub struct Corda {
    note: Note,
    frets: usize,
}

impl Corda {
    pub fn new(note: Note, frets: usize) -> Self {
        Corda { note, frets }
    }
}

pub struct StringInstrument {
    description: String,
    has_bass: bool,
    strings: Vec<Corda>,
}

impl Debug for StringInstrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for StringInstrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instrument({})", self.description)
    }
}

impl StringInstrument {
    pub fn with_bass<S>(description: S, strings: Vec<Corda>) -> Self
    where
        S: AsRef<str>,
    {
        StringInstrument {
            description: description.as_ref().to_owned(),
            has_bass: true,
            strings,
        }
    }
}

lazy_static! {
    pub static ref GUITAR_STANDARD: StringInstrument = StringInstrument::with_bass(
        "Guitar, Standard Tuning",
        vec![
            Corda::new(Note::new(Key::E, 2), 30),
            Corda::new(Note::new(Key::A, 2), 30),
            Corda::new(Note::new(Key::D, 3), 30),
            Corda::new(Note::new(Key::G, 3), 30),
            Corda::new(Note::new(Key::B, 3), 30),
            Corda::new(Note::new(Key::E, 4), 30),
        ]
    );
}

struct Fingering {
    instrument: &'static StringInstrument,
    placements: Vec<Option<usize>>,
}

impl Fingering {
    pub fn to_str(&self) -> String {
        let need_comma = self
            .placements
            .iter()
            .any(|x| matches!(x, Some(n) if *n > 9));
        self.placements
            .iter()
            .map(|x| x.map(|n| n.to_string()).unwrap_or("X".to_owned()))
            .join(if need_comma { "," } else { "" })
    }
}

impl Debug for Fingering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) for {}", self.to_str(), self.instrument)
    }
}

struct BacktrackState {
    placements: Vec<Option<usize>>,
    steps: i64,
    checks: i64,
}

const MAX_DISPLACEMENT: usize = 5;

fn is_in_range(current: &Vec<Option<usize>>, fret: &usize) -> bool {
    let used_frets: Vec<usize> = current
        .iter()
        .filter_map(|f| match f {
            Some(x) if *x > 0 => Some(x),
            _ => None,
        })
        .copied()
        .sorted()
        .collect();

    if used_frets.is_empty() {
        return true;
    }
    let min = used_frets.first().unwrap();
    let max = used_frets.last().unwrap();

    return *fret == 0 || ((*max < MAX_DISPLACEMENT || *fret > max - MAX_DISPLACEMENT) && *fret < min + MAX_DISPLACEMENT);
}

pub fn find_fingerings(chord: &Chord, instrument: &'static StringInstrument) -> Vec<Fingering> {
    let start = SystemTime::now();
    let mut state = BacktrackState {
        placements: vec![],
        steps: 0,
        checks: 0,
    };
    let mut fingerings: Vec<Fingering> = vec![];

    finder_backtrack(chord, instrument, &mut fingerings, &mut state);

    log::debug!(
        "Found {} fingerings for chord:{} ins:{} with {} steps and {} checks, in {} ms",
        fingerings.len(),
        chord,
        instrument,
        state.steps,
        state.checks,
        start.elapsed().unwrap().as_millis(),
    );

    fingerings
}

fn finder_backtrack(
    chord: &Chord,
    instrument: &'static StringInstrument,
    found_fingerings: &mut Vec<Fingering>,
    state: &mut BacktrackState,
) {
    if state.placements.len() >= instrument.strings.len() {
        state.checks += 1;
        if is_valid_fingering(chord, instrument, &state.placements) {
            found_fingerings.push(Fingering {
                instrument,
                placements: state.placements.clone(),
            })
        }
    } else {
        let chord_keys = chord.keys();
        let next_string = &instrument.strings[state.placements.len()];
        backtrap_step(chord, instrument, found_fingerings, state, None);
        let candidates: Vec<usize> = (0..next_string.frets)
            .filter(|f| is_in_range(&state.placements, f))
            .filter(|f| chord_keys.contains(&(next_string.note + *f as i32).key()))
            .collect();

        candidates
            .into_iter()
            .for_each(|fret| backtrap_step(chord, instrument, found_fingerings, state, Some(fret)))
    }
}

fn backtrap_step(
    chord: &Chord,
    instrument: &'static StringInstrument,
    found_fingerings: &mut Vec<Fingering>,
    state: &mut BacktrackState,
    fret: Option<usize>,
) {
    state.placements.push(fret);
    state.steps += 1;
    finder_backtrack(chord, instrument, found_fingerings, state);
    state.placements.pop();
}

fn is_valid_fingering(
    chord: &Chord,
    instrument: &StringInstrument,
    state: &[Option<usize>],
) -> bool {
    let notes: Vec<Note> = instrument
        .strings
        .iter()
        .zip(state)
        .filter_map(|(string, fret)| fret.map(|f| string.note + f as i32))
        .sorted()
        .collect();

    if notes.is_empty() {
        return false;
    }

    if instrument.has_bass && notes[0].key() != chord.bass {
        return false;
    }

    let fingering_keys: HashSet<Key> = notes.iter().map(Note::key).collect();

    return fingering_keys == chord.keys();
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;
    use test_log::test;

    #[test]
    fn exploration() {
        let start = SystemTime::now();
        let chord = &Chord::parse("A7").unwrap();
        let fingerings = find_fingerings(&chord, &GUITAR_STANDARD);
        println!(
            "Found {} fingerings for {} in {:?} ms",
            fingerings.len(),
            chord,
            start.elapsed().map(|x| x.as_millis()).unwrap()
        );
        fingerings
            .iter()
            .filter(|f| f.placements[1] == Some(0))
            .take(10)
            .for_each(|f| {
                dbg!(f);
            });
    }
}
