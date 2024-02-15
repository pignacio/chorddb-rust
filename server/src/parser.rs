use crate::chord::Chord;

#[derive(Debug, PartialEq, Eq)]
pub enum Comp {
    Text(String),
    Chord { chord: Chord, original_text: String },
}

#[derive(Debug, PartialEq, Eq)]
pub struct LineBit {
    pub comp: Comp,
    pub position: usize,
}

pub type Line = Vec<LineBit>;

pub fn parse_tablature<S: AsRef<str>>(source: S) -> Vec<Line> {
    source.as_ref().lines().map(parse_line).collect()
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Whitespace(String),
    Text(String),
}

plex::lexer! {
    fn next_token(text: 'a) -> Token;

    r#"[ \t\r\n]+"# => Token::Whitespace(text.to_owned()),
    r#"[^ \t\r\n]+"# => Token::Text(text.to_owned()),
}

fn tokenize<S: AsRef<str>>(source: S) -> Vec<Token> {
    let mut remaining = source.as_ref();
    let mut tokens: Vec<Token> = Vec::new();
    while let Some((token, new_remaining)) = next_token(remaining) {
        // let final_token = match &token {
        //     Token::Text(text) => Chord::parse(text)
        //         .map(|chord| Token::Chord{chord, original_text: text.to_owned()})
        //         .unwrap_or(token),
        //     _ => token,

        // };
        tokens.push(token);
        remaining = new_remaining;
    }
    tokens
}

fn parse_line<S: AsRef<str>>(source: S) -> Line {
    let mut position = 0;
    let mut line = Line::new();
    let mut current_text = String::new();
    for token in tokenize(source) {
        match &token {
            Token::Whitespace(text) => current_text += text,
            Token::Text(text) => {
                if let Some(chord) = Chord::parse(text) {
                    push_text(&mut current_text, &mut line, &mut position);

                    line.push(LineBit {
                        comp: Comp::Chord {
                            chord,
                            original_text: text.clone(),
                        },
                        position,
                    });
                    position += text.len()
                } else {
                    current_text += text;
                }
            }
        }
    }
    push_text(&mut current_text, &mut line, &mut position);

    line
}

fn push_text(current_text: &mut String, line: &mut Line, position: &mut usize) {
    let trimmed = current_text.trim();
    if !trimmed.is_empty() {
        let mut starting_whitespace = 0;
        for char in current_text.chars() {
            if !char.is_whitespace() {
                break;
            }
            starting_whitespace += 1;
        }
        line.push(LineBit {
            comp: Comp::Text(trimmed.to_owned()),
            position: *position + starting_whitespace,
        });
    }
    *position += current_text.len();
    current_text.clear();
}

#[cfg(test)]
mod tests {
    use crate::chord::{Key, Variant};

    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("I'm a genius"),
            vec![LineBit {
                comp: Comp::Text("I'm a genius".to_owned()),
                position: 0
            }]
        );
        assert_eq!(
            parse_line("I'M A GENIUS"),
            vec![
                LineBit {
                    comp: Comp::Text("I'M".to_owned()),
                    position: 0
                },
                LineBit {
                    comp: Comp::Chord {
                        chord: Chord::simple(Key::A, Variant::Major),
                        original_text: "A".to_owned()
                    },
                    position: 4
                },
                LineBit {
                    comp: Comp::Text("GENIUS".to_owned()),
                    position: 6
                }
            ]
        );
        assert_eq!(
            parse_line("C#7/G#"),
            vec![LineBit {
                comp: Comp::Chord {
                    chord: Chord::new(Key::Db, Variant::Seventh, Key::Ab),
                    original_text: "C#7/G#".to_owned()
                },
                position: 0
            }]
        );
        assert_eq!(
            parse_line(" trailing A7 whitespace "),
            vec![
                LineBit {
                    comp: Comp::Text("trailing".to_owned()),
                    position: 1
                },
                LineBit {
                    comp: Comp::Chord {
                        chord: Chord::simple(Key::A, Variant::Seventh),
                        original_text: "A7".to_owned()
                    },
                    position: 10
                },
                LineBit {
                    comp: Comp::Text("whitespace".to_owned()),
                    position: 13
                }
            ]
        );
        // assert_eq!(parsed.len(), 5);
        // assert_eq!(parsed[0].position, 0);
        // assert_eq!(parsed[0].comp, Comp::Chord { chord: Chord::parse("A").unwrap() })
    }
}
