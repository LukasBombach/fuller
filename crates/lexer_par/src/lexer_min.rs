use crate::cursor::Cursor;
use crate::cursor::EOF_CHAR;
use crate::token::TokenKind::*;
use crate::token::*;

pub(crate) struct Lexer<'a> {
  cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
  pub(crate) fn new(input: &'a str) -> Self {
    let cursor = Cursor::new(input);
    Lexer { cursor }
  }
}

impl<'a> Lexer<'a> {
  fn get_token(&mut self) -> Token {
    let first_char = self.cursor.next_char();

    let kind = match first_char {
      _ => Unknown,
    };
  }
}
