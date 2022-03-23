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
  fn next_token(&mut self) -> Token {
    let start_pos = self.cursor.current_pos();
    let next_char = self.cursor.next_char();
    let token_kind = match next_char {
      EOF_CHAR => Eof,
    };
    let end_pos = self.cursor.current_pos();
    Token::new(token_kind, end_pos - start_pos)
  }
}
