use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chorddb::chord::{
    finder::{find_fingerings, GUITAR_STANDARD},
    Chord, Key, Variant,
};
use lazy_static::lazy_static;
use sea_orm::Iterable;

lazy_static! {
    static ref ALL_CHORDS_BY_VARIANT: HashMap<Variant, Vec<String>> = Variant::iter()
        .map(|v| (
            v,
            Key::iter()
                .map(|k| format!("{}{}", k.text(), v.text()))
                .collect()
        ))
        .collect();
}

fn parse_all_chords(texts: &[String]) {
    for text in texts {
        black_box(Chord::parse(text));
    }
}

fn find_all_fingerings(variant: &Variant, bass: Option<Key>) {
    for key in Key::iter() {
        find_fingerings(
            &Chord::new(key, *variant, bass.unwrap_or(key)),
            &GUITAR_STANDARD,
        );
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    for variant in Variant::iter() {
        c.bench_function(&format!("parse all {:?} chords", variant), |b| {
            b.iter(|| parse_all_chords(black_box(&ALL_CHORDS_BY_VARIANT[&Variant::Major])))
        });
    }
    for variant in Variant::iter() {
        c.bench_function(
            &format!("find fingerings for all {:?} chords, no bass", variant),
            |b| b.iter(|| find_all_fingerings(black_box(&variant), None)),
        );
        c.bench_function(
            &format!("find fingerings for all {:?} chords, C bass", variant),
            |b| b.iter(|| find_all_fingerings(black_box(&variant), Some(Key::C))),
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
