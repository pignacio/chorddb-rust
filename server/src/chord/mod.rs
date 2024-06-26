use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Add,
};

use itertools::Itertools;
use regex::Regex;
use sea_orm::Iterable;
use strum::EnumIter;

pub mod finder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter)]
pub enum Key {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

lazy_static! {
    static ref ALL_KEYS: Vec<Key> = Key::iter().collect();
    static ref KEYS_BY_NAME: HashMap<&'static str, Key> = Key::iter()
        .flat_map(|k| k
            .valid_names()
            .into_iter()
            .map(|n| (n, k))
            .collect::<Vec<_>>())
        .collect();
}

impl Key {
    fn parse(source: &str) -> Option<&'static Key> {
        KEYS_BY_NAME.get(source)
    }

    fn ordinal(&self) -> usize {
        match self {
            Key::C => 0,
            Key::Db => 1,
            Key::D => 2,
            Key::Eb => 3,
            Key::E => 4,
            Key::F => 5,
            Key::Gb => 6,
            Key::G => 7,
            Key::Ab => 8,
            Key::A => 9,
            Key::Bb => 10,
            Key::B => 11,
        }
    }

    pub const fn text(&self) -> &'static str {
        match self {
            Key::C => "C",
            Key::Db => "Db",
            Key::D => "D",
            Key::Eb => "Eb",
            Key::E => "E",
            Key::F => "F",
            Key::Gb => "Gb",
            Key::G => "G",
            Key::Ab => "Ab",
            Key::A => "A",
            Key::Bb => "Bb",
            Key::B => "B",
        }
    }

    pub fn valid_names(&self) -> Vec<&'static str> {
        match self {
            Key::C => vec!["C"],
            Key::Db => vec!["C#", "Db"],
            Key::D => vec!["D"],
            Key::Eb => vec!["D#", "Eb"],
            Key::E => vec!["E"],
            Key::F => vec!["F"],
            Key::Gb => vec!["F#", "Gb"],
            Key::G => vec!["G"],
            Key::Ab => vec!["G#", "Ab"],
            Key::A => vec!["A"],
            Key::Bb => vec!["A#", "Bb"],
            Key::B => vec!["B"],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Note {
    key: Key,
    octave: i32,
}

impl Note {
    fn new(key: Key, octave: i32) -> Self {
        Note { key, octave }
    }

    fn ordinal(&self) -> i32 {
        self.key.ordinal() as i32 + 12 * self.octave
    }

    fn from_ordinal(ordinal: i32) -> Self {
        let mut octave = ordinal / 12;
        let mut key_ordinal = ordinal % 12;
        while key_ordinal < 0 {
            key_ordinal += 12;
            octave -= 1;
        }
        Note {
            key: ALL_KEYS[key_ordinal as usize],
            octave,
        }
    }

    pub fn key(&self) -> Key {
        self.key
    }

    pub fn octave(&self) -> i32 {
        self.octave
    }

    pub fn text(&self) -> String {
        format!("{}{}", self.key.text(), self.octave)
    }
}

impl Add<i32> for Note {
    type Output = Note;

    fn add(self, rhs: i32) -> Self::Output {
        Note::from_ordinal(self.ordinal() + rhs)
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ordinal().cmp(&other.ordinal())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Variant {
    Major,
    Minor,
    Seventh,
    MinorSeventh,
    MinorSixth,
    SuspendedSecond,
    AddNinth,
    Diminished,
    Augmented,
}

lazy_static! {
    static ref VARIANTS_BY_TEXT: HashMap<&'static str, Variant> =
        Variant::iter().map(|v| (v.text(), v)).collect();
}

impl Variant {
    pub fn parse(source: &str) -> Option<&'static Variant> {
        VARIANTS_BY_TEXT.get(source)
    }

    pub fn text(&self) -> &'static str {
        match self {
            Variant::Major => "",
            Variant::Minor => "m",
            Variant::Seventh => "7",
            Variant::MinorSeventh => "m7",
            Variant::MinorSixth => "m6",
            Variant::SuspendedSecond => "sus2",
            Variant::AddNinth => "add9",
            Variant::Diminished => "dim",
            Variant::Augmented => "aug",
        }
    }

    pub fn intervals(&self) -> Vec<usize> {
        match self {
            Variant::Major => vec![0, 4, 7],
            Variant::Minor => vec![0, 3, 7],
            Variant::Seventh => vec![0, 4, 7, 10],
            Variant::MinorSeventh => vec![0, 3, 7, 10],
            Variant::MinorSixth => vec![0, 3, 7, 9],
            Variant::SuspendedSecond => vec![0, 2, 7],
            Variant::AddNinth => vec![0, 4, 7, 14],
            Variant::Diminished => vec![0, 3, 6],
            Variant::Augmented => vec![0, 4, 8],
        }
    }
}

lazy_static! {
    static ref KEY_PATTERN: String = format!("(?:{})", KEYS_BY_NAME.keys().join("|"));
    static ref VARIANT_PATTERN: String =
        format!("(?:{})", Variant::iter().map(|v| v.text()).join("|"));
    static ref CHORD_PATTERN: String = format!(
        "^(?<root>{})(?<variant>{})(?:/(?<bass>{}))?$",
        *KEY_PATTERN, *VARIANT_PATTERN, *KEY_PATTERN
    );
    static ref CHORD_REGEX: Regex = Regex::new(&CHORD_PATTERN).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chord {
    root: Key,
    variant: Variant,
    bass: Key,
}

impl Chord {
    pub fn simple(root: Key, variant: Variant) -> Self {
        Chord::new(root, variant, root)
    }

    pub fn new(root: Key, variant: Variant, bass: Key) -> Self {
        Chord {
            root,
            variant,
            bass,
        }
    }

    pub fn text(&self) -> String {
        let bass = if self.root == self.bass {
            "".to_owned()
        } else {
            format!("/{}", self.bass.text())
        };
        format!("{}{}{}", self.root.text(), self.variant.text(), bass)
    }

    pub fn parse<S: AsRef<str>>(source: S) -> Option<Chord> {
        log::trace!("Parsing chord '{}'", source.as_ref());
        let caps = CHORD_REGEX.captures(source.as_ref())?;
        let root = Key::parse(&caps["root"]).copied()?;
        Some(Chord {
            root,
            variant: Variant::parse(&caps["variant"]).copied()?,
            bass: caps
                .name("bass")
                .and_then(|s| Key::parse(s.as_str()))
                .cloned()
                .unwrap_or(root),
        })
    }

    pub fn keys(&self) -> HashSet<Key> {
        let root_ordinal = self.root.ordinal();
        self.variant
            .intervals()
            .iter()
            .map(|interval| (root_ordinal + interval) % 12)
            .map(|o| ALL_KEYS[o])
            .collect()
    }

    pub fn keys_with_bass(&self) -> HashSet<Key> {
        let mut keys = self.keys();
        keys.insert(self.bass);
        keys
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chord({})", self.text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn exploration() {
        println!("{:?}", Chord::parse("A"));
        println!("{:?}", Chord::parse("Bm"));
        println!("{:?}", Chord::parse("C/D"));
        println!("{:?}", Chord::parse("Db7/Ab"));
        println!("{:?}", Chord::parse("C#7/G#"));
        println!("{:?}", Chord::parse("SomeRandomString"));
    }

    #[test]
    fn validate_key_ordinals() {
        for key in Key::iter() {
            assert_eq!(key, ALL_KEYS[key.ordinal()]);
        }
    }

    #[test]
    fn validate_key_parsing() {
        for key in Key::iter() {
            assert_eq!(key, *Key::parse(key.text()).unwrap());
        }
    }

    fn parse_chord(text: String) -> Option<Chord> {
        let chord = Chord::parse(&text);
        log::trace!("Parsed '{}' into {:?}", &text, chord);
        chord
    }

    #[test]
    fn validate_all_chords_parse() {
        for root in Key::iter() {
            for variant in Variant::iter() {
                let chord = parse_chord(format!("{}{}", root.text(), variant.text())).unwrap();
                assert_eq!(chord.root, root);
                assert_eq!(chord.variant, variant);
                assert_eq!(chord.bass, root);

                for bass in Key::iter() {
                    let chord =
                        parse_chord(format!("{}{}/{}", root.text(), variant.text(), bass.text()))
                            .unwrap();
                    assert_eq!(chord.root, root);
                    assert_eq!(chord.variant, variant);
                    assert_eq!(chord.bass, bass);
                }
            }
        }
    }
}
