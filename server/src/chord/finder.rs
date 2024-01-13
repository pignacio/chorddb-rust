use sorted_vec::SortedVec;
use std::alloc::System;
use std::cmp::min;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::time::SystemTime;

use itertools::Itertools;

use super::Chord;
use super::Key;
use super::Note;

#[derive(Debug)]
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

struct BacktrackState<'a> {
    instrument: &'a StringInstrument,
    chord_keys: HashSet<Key>,
    placements: Vec<Option<usize>>,
    sorted_placements: SortedVec<usize>,
    sorted_notes: SortedVec<Note>,
    sorted_keys: SortedVec<Key>,
    steps: i64,
    checks: i64,
    no_notes: i64,
    bad_bass: i64,
    bad_notes: i64,
}

impl<'a> BacktrackState<'a> {
    fn push_placement(&mut self, placement: Option<usize>) {
        self.placements.push(placement);
        if let Some(new_placement) = placement {
            let string = &self.instrument.strings[self.placements.len() - 1];
            self.sorted_placements.push(new_placement);
            let note = string.note + new_placement as i32;
            self.sorted_notes.push(note);
            self.sorted_keys.push(note.key());
            log::trace!(
                "Adding placement on string {:?} fret {}. note: {:?}, key: {:?}",
                string,
                new_placement,
                note,
                note.key()
            );
        }
    }

    fn pop_placement(&mut self) {
        let popped = self.placements.pop().flatten();
        if let Some(removing) = popped {
            let string = &self.instrument.strings[self.placements.len()];
            let note = string.note + removing as i32;
            log::trace!(
                "Removing placement on string {:?} fret {}. note: {:?}, key: {:?}",
                string,
                removing,
                note,
                note.key()
            );
            assert_ne!(self.sorted_placements.remove_item(&removing), None);
            assert_ne!(self.sorted_notes.remove_item(&note), None);
            assert_ne!(self.sorted_keys.remove_item(&note.key()), None);
        }
    }

    fn find_index_for(&self, placement: usize) -> usize {
        let mut index = 0;
        for existing_placement in self.sorted_placements.iter() {
            if existing_placement >= &placement {
                break;
            }
            index += 1;
        }
        index
    }

    fn first_fingered_placement(&self) -> Option<&usize> {
        self.sorted_placements.first()
    }

    fn last_fingered_placement(&self) -> Option<&usize> {
        self.sorted_placements.last()
    }
}

const MAX_DISPLACEMENT: usize = 4;

fn is_in_range(state: &BacktrackState, fret: &usize) -> bool {
    let Some(min) = state.first_fingered_placement() else {
        return true;
    };
    let Some(max) = state.last_fingered_placement() else {
        return true;
    };

    return *fret == 0
        || ((*max <= MAX_DISPLACEMENT || *fret >= max - MAX_DISPLACEMENT)
            && *fret <= min + MAX_DISPLACEMENT);
}

pub fn find_fingerings(chord: &Chord, instrument: &'static StringInstrument) -> Vec<Fingering> {
    let start = SystemTime::now();
    let mut chord_keys = chord.keys();
    chord_keys.insert(chord.bass);
    let candidates: Vec<Vec<usize>> = instrument
        .strings
        .iter()
        .map(|string| {
            (0..string.frets)
                .filter(|f| chord_keys.contains(&(string.note + *f as i32).key()))
                .collect()
        })
        .collect();

    let mut state = BacktrackState {
        instrument: instrument,
        chord_keys: chord.keys().iter().copied().collect(),
        placements: vec![],
        sorted_placements: SortedVec::new(),
        sorted_notes: SortedVec::new(),
        sorted_keys: SortedVec::new(),
        steps: 0,
        checks: 0,
        no_notes: 0,
        bad_bass: 0,
        bad_notes: 0,
    };
    let mut fingerings: Vec<Fingering> = vec![];

    finder_backtrack(chord, instrument, &mut fingerings, &candidates, &mut state);

    log::debug!(
        "Found {} fingerings for chord:{} ins:{} with {} steps and {} checks, in {} us (no notes:{}, bad bass:{}, bad notes:{})",
        fingerings.len(),
        chord,
        instrument,
        state.steps,
        state.checks,
        start.elapsed().unwrap().as_micros(),
        state.no_notes,
        state.bad_bass,
        state.bad_notes,
        // "UNKNOWN",
    );

    // log::debug!("Average check duration: {}ns", state.checks.iter().sum::<u128>() / state.checks.len() as u128);

    fingerings
}

fn finder_backtrack(
    chord: &Chord,
    instrument: &'static StringInstrument,
    found_fingerings: &mut Vec<Fingering>,
    candidates: &Vec<Vec<usize>>,
    state: &mut BacktrackState,
) {
    if state.placements.len() >= instrument.strings.len() {
        state.checks += 1;
        if is_valid_fingering(chord, instrument, state) {
            found_fingerings.push(Fingering {
                instrument,
                placements: state.placements.clone(),
            })
        }
    } else {
        backtrap_step(chord, instrument, found_fingerings, candidates, state, None);
        let index = state.placements.len();
        for candidate in &candidates[index] {
            if is_in_range(&state, candidate) {
                backtrap_step(
                    chord,
                    instrument,
                    found_fingerings,
                    candidates,
                    state,
                    Some(*candidate),
                );
            }
        }
    }
}

fn backtrap_step(
    chord: &Chord,
    instrument: &'static StringInstrument,
    found_fingerings: &mut Vec<Fingering>,
    candidates: &Vec<Vec<usize>>,
    state: &mut BacktrackState,
    fret: Option<usize>,
) {
    state.push_placement(fret);
    state.steps += 1;
    finder_backtrack(chord, instrument, found_fingerings, candidates, state);
    state.pop_placement();
}

fn is_valid_fingering(
    chord: &Chord,
    instrument: &StringInstrument,
    state: &mut BacktrackState,
) -> bool {
    if state.sorted_notes.is_empty() {
        // state.no_notes += 1;
        return false;
    }

    let mut bass = state.sorted_notes.first().unwrap();

    if instrument.has_bass && bass.key() != chord.bass {
        log::trace!(
            "Expected bass in {:?} ({:?}) to be {:?} but got {:?}",
            state.placements,
            state.sorted_notes,
            chord.bass,
            bass.key()
        );
        // state.bad_bass += 1;
        return false;
    }
    
    let fingering_keys: HashSet<Key> = state.sorted_keys.iter()
        .skip(if chord.root == chord.bass { 0 } else { 1 })
        .copied()
        .collect();
    if fingering_keys == state.chord_keys {
        return true;
    } else {
        // state.bad_notes += 1;
        return false;
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;
    use test_log::test;

    #[test]
    fn exploration() {
        let chords = [
            Chord::parse("A7").unwrap(),
            Chord::parse("A").unwrap(),
            Chord::parse("Am").unwrap(),
            Chord::parse("Bb").unwrap(),
            Chord::parse("B").unwrap(),
            Chord::parse("C").unwrap(),
            Chord::parse("Db").unwrap(),
            Chord::parse("D").unwrap(),
            Chord::parse("Eb").unwrap(),
            Chord::parse("E").unwrap(),
            Chord::parse("F").unwrap(),
            Chord::parse("Gb").unwrap(),
            Chord::parse("G").unwrap(),
            Chord::parse("Ab").unwrap(),
            Chord::parse("Dbm").unwrap(),
            Chord::parse("Db7").unwrap(),
            Chord::parse("Db7/C").unwrap(),
        ];
        let count = 1;
        
        for chord in chords {
            let start = SystemTime::now();
            let mut fingerings = vec![];
            (0..count).into_iter().for_each(|_| {
                fingerings = find_fingerings(&chord, &GUITAR_STANDARD);
            });
            let elapsed = start.elapsed().unwrap();
            println!(
                "Found {} fingerings ({} times) for {} in {:?} ms ({} nanos/search)",
                fingerings.len(),
                count,
                chord,
                elapsed.as_millis(),
                elapsed.as_nanos() / count,
            );
            fingerings
                .iter()
                //.filter(|f| f.placements[0] != None)
                .take(10)
                .for_each(|f| {
                    dbg!(f);
                });
        };
    }
}