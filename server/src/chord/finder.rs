use sorted_vec::SortedVec;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::time::SystemTime;

use itertools::Itertools;

use crate::chord::Variant;

use super::Chord;
use super::Key;
use super::Note;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Corda {
    note: Note,
    frets: usize,
}

impl Corda {
    pub fn new(note: Note, frets: usize) -> Self {
        Corda { note, frets }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct StringInstrument {
    id: String,
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
    pub fn with_bass<S>(id: S, description: S, strings: Vec<Corda>) -> Self
    where
        S: AsRef<str>,
    {
        StringInstrument {
            id: id.as_ref().to_owned(),
            description: description.as_ref().to_owned(),
            has_bass: true,
            strings,
        }
    }

    pub fn without_bass<S>(id: S, description: S, strings: Vec<Corda>) -> Self
    where
        S: AsRef<str>,
    {
        StringInstrument {
            id: id.as_ref().to_owned(),
            description: description.as_ref().to_owned(),
            has_bass: false,
            strings,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

lazy_static! {
    pub static ref GUITAR_STANDARD: StringInstrument = StringInstrument::with_bass(
        "guitar",
        "Guitar, Standard Tuning",
        vec![
            Corda::new(Note::new(Key::E, 2), 24),
            Corda::new(Note::new(Key::A, 2), 24),
            Corda::new(Note::new(Key::D, 3), 24),
            Corda::new(Note::new(Key::G, 3), 24),
            Corda::new(Note::new(Key::B, 3), 24),
            Corda::new(Note::new(Key::E, 4), 24),
        ]
    );
    pub static ref MIMI: StringInstrument = StringInstrument::without_bass(
        "mimi",
        "Loog Guitar",
        vec![
            Corda::new(Note::new(Key::G, 3), 16),
            Corda::new(Note::new(Key::B, 3), 16),
            Corda::new(Note::new(Key::E, 4), 16),
        ]
    );
    pub static ref THREE_STRING_DOWNGRADES: HashMap<Variant, Variant> = {
        let mut m = HashMap::new();
        m.insert(Variant::MinorSixth, Variant::Minor);
        m.insert(Variant::MinorSeventh, Variant::Minor);
        m.insert(Variant::Seventh, Variant::Major);
        m.insert(Variant::AddNinth, Variant::Major);
        m
    };
}

#[derive(Clone)]
pub struct Fingering {
    instrument_id: String,
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

    pub fn placements(&self) -> &[Option<usize>] {
        &self.placements
    }
}

impl Debug for Fingering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) for Instrument<{}>",
            self.to_str(),
            self.instrument_id
        )
    }
}

struct BacktrackState {
    instrument: StringInstrument,
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

impl BacktrackState {
    fn starting(chord: &Chord, instrument: &StringInstrument) -> BacktrackState {
        BacktrackState {
            instrument: instrument.clone(),
            chord_keys: chord.keys(),
            placements: vec![],
            sorted_placements: SortedVec::new(),
            sorted_notes: SortedVec::new(),
            sorted_keys: SortedVec::new(),
            steps: 0,
            checks: 0,
            no_notes: 0,
            bad_bass: 0,
            bad_notes: 0,
        }
    }

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

    *fret == 0
        || ((*max <= MAX_DISPLACEMENT || *fret >= max - MAX_DISPLACEMENT)
            && *fret <= min + MAX_DISPLACEMENT)
}

pub fn find_fingerings(chord: &Chord, instrument: &StringInstrument) -> Vec<Fingering> {
    if !instrument.has_bass && chord.bass != chord.root {
        let new_chord = Chord::new(chord.root, chord.variant, chord.root);
        log::info!(
            "Downgrading chord {} to {} because Instrument<{}> does not have a bass",
            chord,
            new_chord,
            instrument.id()
        );
        return find_fingerings(&new_chord, instrument);
    }
    if instrument.strings.len() < 4 {
        if let Some(new_variant) = THREE_STRING_DOWNGRADES.get(&chord.variant) {
            let new_chord = Chord::new(chord.root, *new_variant, chord.root);
            log::info!(
                "Downgrading chord {} to {} because Instrument<{}> does not have a bass",
                chord,
                new_chord,
                instrument.id()
            );
            return find_fingerings(&new_chord, instrument);
        }
    }
    let start = SystemTime::now();
    let chord_keys = chord.keys();
    let candidates: Vec<Vec<usize>> = instrument
        .strings
        .iter()
        .map(|string| {
            (0..string.frets)
                .filter(|f| chord_keys.contains(&(string.note + *f as i32).key()))
                .collect()
        })
        .collect();

    let mut state = BacktrackState::starting(chord, instrument);
    let mut fingerings: Vec<Fingering> = vec![];

    finder_backtrack(chord, instrument, &mut fingerings, &candidates, &mut state);

    log::trace!(
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
    instrument: &StringInstrument,
    found_fingerings: &mut Vec<Fingering>,
    candidates: &Vec<Vec<usize>>,
    state: &mut BacktrackState,
) {
    if state.placements.len() >= instrument.strings.len() {
        state.checks += 1;
        if let Ok(fingering) = get_valid_fingering(chord, instrument, state) {
            found_fingerings.push(fingering);
        }
    } else {
        let index = state.placements.len();
        for candidate in &candidates[index] {
            if is_in_range(state, candidate) {
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
        backtrap_step(chord, instrument, found_fingerings, candidates, state, None);
    }
}

fn backtrap_step(
    chord: &Chord,
    instrument: &StringInstrument,
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

fn get_valid_fingering(
    chord: &Chord,
    instrument: &StringInstrument,
    state: &mut BacktrackState,
) -> Result<Fingering, String> {
    if state.sorted_notes.is_empty() {
        // state.no_notes += 1;
        return Err("No notes".into());
    }

    let bass = state.sorted_notes.first().unwrap();

    if instrument.has_bass && bass.key() != chord.bass {
        log::trace!(
            "Expected bass in {:?} ({:?}) to be {:?} but got {:?}",
            state.placements,
            state.sorted_notes,
            chord.bass,
            bass.key()
        );
        // state.bad_bass += 1;
        return Err(format!(
            "Bass does not match. Expected {:?} but got {:?}",
            chord.bass, bass
        ));
    }

    let fingering_keys: HashSet<Key> = state
        .sorted_keys
        .iter()
        .skip(if chord.root == chord.bass { 0 } else { 1 })
        .copied()
        .collect();
    if fingering_keys == state.chord_keys {
        Ok(Fingering {
            instrument_id: state.instrument.id().to_string(),
            placements: state.placements.clone(),
        })
    } else {
        // state.bad_notes += 1;
        Err(format!(
            "Bad notes! Expected {:?} but got {:?}",
            state.chord_keys, fingering_keys
        ))
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
            (0..count).for_each(|_| {
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
        }
    }

    fn build_state_for(
        chord: &Chord,
        instrument: &'static StringInstrument,
        placements: Vec<Option<usize>>,
    ) -> BacktrackState {
        let mut state = BacktrackState::starting(chord, instrument);
        for placement in placements {
            state.push_placement(placement);
        }
        state
    }

    #[test]
    fn test_is_valid_fingering() {
        let chord = Chord::parse("Cadd9").expect("Invalid chord");
        let fingering = vec![None, Some(3), Some(2), Some(0), Some(3), Some(0)];
        let mut state = build_state_for(&chord, &GUITAR_STANDARD, fingering.clone());
        if let Err(msg) = get_valid_fingering(&chord, &GUITAR_STANDARD, &mut state) {
            panic!(
                "Expected fingering {:?} to be valid for {} but it was not. Error: {}",
                fingering, chord, msg
            )
        }
    }
}
