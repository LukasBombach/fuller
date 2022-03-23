use crate::cursor::Cursor;
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
  fn next_token(&mut self) -> Token {
    let next_char = self.cursor.next_char()
  }
}
