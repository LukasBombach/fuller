use crate::position::Position;
use std::str::Chars;

pub const EOF_CHAR: char = '\0';

pub struct Cursor<'a> {
  chars: Chars<'a>,
  // len: usize,
  pub pos: Position,
}

impl Cursor<'_> {
  pub fn newline(&mut self) {
    self.pos.line += 1;
    self.pos.col = 0;
  }

  pub fn advance(&mut self) {
    self.pos.col += 1;
  }
}

impl<'a> Cursor<'a> {
  pub fn new(input: &'a str) -> Cursor<'a> {
    Cursor {
      chars: input.chars(),
      // len: input.len(),
      pos: Position::new(),
    }
  }

  pub fn next_char(&mut self) -> char {
    self.chars.next().unwrap_or(EOF_CHAR)
  }

  pub fn next_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    while predicate(self.peek()) {
      self.chars.next();
    }
  }

  pub fn peek(&self) -> char {
    self.chars.clone().next().unwrap_or(EOF_CHAR)
  }

  /*

  pub fn next(&mut self) -> Option<char> {
    let c = self.chars.next()?;
    self.pos += self.len_consumed();
    Some(c)
  }

  pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    while predicate(self.peek()) && !self.is_eof() {
      self.chars.next();
    }
    self.pos += self.len_consumed()
  }

  pub fn is_eof(&self) -> bool {
    self.chars.as_str().is_empty()
  }

  pub fn len_consumed(&self) -> usize {
    self.len - self.pos - self.chars.as_str().len()
  } */
}

/* impl<'a> Cursor<'a> {
  pub fn next_non_whitespace(&mut self) -> char {
    self.next_while(|c| c.is_whitespace());
    self.next_char()
  }
} */
