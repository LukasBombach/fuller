mod cursor;
mod position;
mod token;

pub use crate::cursor::Cursor;
pub use crate::position::*;
pub use crate::token::*;

use crate::token::TokenKind;
use crate::token::TokenKind::*;

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let cursor = Cursor::new(input);
        Lexer { cursor }
    }
}

impl<'a> Lexer<'a> {
    pub fn next_token(&mut self) -> Token {
        let first_char = self.cursor.next_char();
        let start = self.cursor.pos.clone();

        let kind = match first_char {
            'c' => Const,
            'a'..='z' => Ident,
            _ => Unknown,
        };

        let end = self.cursor.pos.clone();

        Token { kind, start, end }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn first_goalpost() {
        let mut lexer = Lexer::new("const val = true;");
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: Const,
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 6 }
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: Ident,
                start: Position { line: 1, col: 8 },
                end: Position { line: 1, col: 11 }
            }
        );
    }
}
