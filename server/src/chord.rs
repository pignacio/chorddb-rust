use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    A,
    Bb,
    B,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
}

// This must honor key == ALL_KEYS[key.ordinal()]
const ALL_KEYS: [Key; 12] = [
    Key::A,
    Key::Bb,
    Key::B,
    Key::C,
    Key::Db,
    Key::D,
    Key::Eb,
    Key::E,
    Key::F,
    Key::Gb,
    Key::G,
    Key::Ab,
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
            Key::A => 0,
            Key::Bb => 1,
            Key::B => 2,
            Key::C => 3,
            Key::Db => 4,
            Key::D => 5,
            Key::Eb => 6,
            Key::E => 7,
            Key::F => 8,
            Key::Gb => 9,
            Key::G => 10,
            Key::Ab => 11,
        }
    }

    pub const fn text(&self) -> &'static str {
        match self {
            Key::A => "A",
            Key::Bb => "Bb",
            Key::B => "B",
            Key::C => "C",
            Key::Db => "Db",
            Key::D => "D",
            Key::Eb => "Eb",
            Key::E => "E",
            Key::F => "F",
            Key::Gb => "Gb",
            Key::G => "G",
            Key::Ab => "Ab",
        }
    }

    pub fn valid_names(&self) -> Vec<&'static str> {
        match self {
            Key::A => vec!["A"],
            Key::Bb => vec!["A#", "Bb"],
            Key::B => vec!["B"],
            Key::C => vec!["C"],
            Key::Db => vec!["C#", "Db"],
            Key::D => vec!["D"],
            Key::Eb => vec!["D#", "Eb"],
            Key::E => vec!["E"],
            Key::F => vec!["F"],
            Key::Gb => vec!["F#", "Gb"],
            Key::G => vec!["G"],
            Key::Ab => vec!["G#", "Ab"],
        }
    }
}

pub struct Note {
    key: Key,
    octave: i8,
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
        Chord{ root, variant, bass}
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
        log::info!("Parsed '{}' into {:?}", &text, chord);
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
