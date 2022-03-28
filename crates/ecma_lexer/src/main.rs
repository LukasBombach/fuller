use std::str::Chars;
use TokenKind::*;

#[derive(Debug)]
enum TokenKind {
    CONST,
    IDENT,
    EOF,
    UNEXPECTED,
}

pub struct Cursor<'a> {
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
          chars: input.chars(),
        }
    }

    pub fn next(&mut self) -> char {
        self.chars.next().unwrap_or_default()
    }
}

fn main() {
    let input = "const conrad";
    let mut tokens =  Vec::new();
    let mut cursor = Cursor::new(input);

    loop {
        match cursor.next() {
            'c' => match cursor.next() {
                'o' => match cursor.next() {
                    'n' => match cursor.next() {
                        's' => match cursor.next() {
                            't' => tokens.push(CONST),
                            _ => tokens.push(IDENT),
                        },
                        _ => tokens.push(IDENT),
                    },
                    _ => tokens.push(IDENT),
                },
                _ => tokens.push(IDENT),
            }
            '\x00' => {
                tokens.push(EOF);
                break;
            }
            _ => tokens.push(UNEXPECTED)
        }

    }

    println!("{:#?}", tokens);
}
