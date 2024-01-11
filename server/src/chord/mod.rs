use std::{collections::{HashMap, HashSet}, fmt::Display, ops::Add};

use itertools::Itertools;
use regex::Regex;

pub mod finder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

// This must honor key == ALL_KEYS[key.ordinal()]
const ALL_KEYS: [Key; 12] = [
    Key::C,
    Key::Db,
    Key::D,
    Key::Eb,
    Key::E,
    Key::F,
    Key::Gb,
    Key::G,
    Key::Ab,
    Key::A,
    Key::Bb,
    Key::B,
];

lazy_static! {
    static ref KEYS_BY_NAME: HashMap<&'static str, &'static Key> = ALL_KEYS
        .iter()
        .map(|k| k
            .valid_names()
            .into_iter()
            .map(|n| (n, k))
            .collect::<Vec<_>>())
        .flatten()
        .collect();
}

impl Key {
    fn parse(source: &str) -> Option<&'static Key> {
        KEYS_BY_NAME.get(source).copied()
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
        return self.key.ordinal() as i32 + 12 * self.octave;
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
}

impl Add<i32> for Note {
    type Output = Note;

    fn add(self, rhs: i32) -> Self::Output {
        return Note::from_ordinal(self.ordinal() + rhs);
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(self, &other))
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ordinal().cmp(&other.ordinal())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Major,
    Minor,
    Seventh,
}

const ALL_VARIANTS: [Variant; 3] = [Variant::Major, Variant::Minor, Variant::Seventh];

lazy_static! {
    static ref VARIANTS_BY_TEXT: HashMap<&'static str, &'static Variant> =
        ALL_VARIANTS.iter().map(|v| (v.text(), v)).collect();
}

impl Variant {
    pub fn parse(source: &str) -> Option<&'static Variant> {
        VARIANTS_BY_TEXT.get(source).copied()
    }

    pub fn text(&self) -> &'static str {
        match self {
            Variant::Major => "",
            Variant::Minor => "m",
            Variant::Seventh => "7",
        }
    }

    pub fn intervals(&self) -> Vec<usize> {
        match self {
            Variant::Major => vec![0, 4, 7],
            Variant::Minor => vec![0, 3, 7],
            Variant::Seventh => vec![0, 4, 7, 10],
        }
    }
}

lazy_static! {
    static ref KEY_PATTERN: String = format!("(?:{})", KEYS_BY_NAME.keys().join("|"));
    static ref VARIANT_PATTERN: String =
        format!("(?:{})", ALL_VARIANTS.iter().map(|v| v.text()).join("|"));
    static ref CHORD_PATTERN: String = format!(
        "^(?<root>{})(?<variant>{})(?:/(?<bass>{}))?$",
        *KEY_PATTERN, *VARIANT_PATTERN, *KEY_PATTERN
    );
    static ref CHORD_REGEX: Regex = Regex::new(&*CHORD_PATTERN).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let Some(caps) = CHORD_REGEX.captures(source.as_ref()) else {
            return None;
        };
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
        for key in &ALL_KEYS {
            assert_eq!(*key, ALL_KEYS[key.ordinal()]);
        }
    }

    #[test]
    fn validate_key_parsing() {
        for key in &ALL_KEYS {
            assert_eq!(key, Key::parse(key.text()).unwrap());
        }
    }

    fn parse_chord(text: String) -> Option<Chord> {
        let chord = Chord::parse(&text);
        log::trace!("Parsed '{}' into {:?}", &text, chord);
        return chord;
    }

    #[test]
    fn validate_all_chords_parse() {
        for root in &ALL_KEYS {
            for variant in &ALL_VARIANTS {
                let chord = parse_chord(format!("{}{}", root.text(), variant.text())).unwrap();
                assert_eq!(&chord.root, root);
                assert_eq!(&chord.variant, variant);
                assert_eq!(&chord.bass, root);

                for bass in &ALL_KEYS {
                    let chord =
                        parse_chord(format!("{}{}/{}", root.text(), variant.text(), bass.text()))
                            .unwrap();
                    assert_eq!(&chord.root, root);
                    assert_eq!(&chord.variant, variant);
                    assert_eq!(&chord.bass, bass);
                }
            }
        }
    }
}
