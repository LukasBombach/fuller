use crate::position::Position;
use std::str::Chars;

pub(crate) const EOF_CHAR: char = '\0';

pub(crate) struct Cursor<'a> {
  chars: Chars<'a>,
  len: usize,
  pub(crate) pos: Position,
}

impl Cursor<'_> {
  pub(crate) fn newline(&mut self) {
    self.pos.line += 1;
    self.pos.col = 0;
  }

  pub(crate) fn advance(&mut self) {
    self.pos.col += 1;
  }
}

impl<'a> Cursor<'a> {
  pub(crate) fn new(input: &'a str) -> Cursor<'a> {
    Cursor {
      chars: input.chars(),
      len: input.len(),
      pos: Position::new(),
    }
  }

  pub(crate) fn next_char(&mut self) -> char {
    self.chars.next().unwrap_or(EOF_CHAR)
  }

  pub(crate) fn next_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    while predicate(self.peek()) {
      self.chars.next();
    }
  }

  pub(crate) fn peek(&self) -> char {
    self.chars.clone().next().unwrap_or(EOF_CHAR)
  }

  /*

  pub(crate) fn next(&mut self) -> Option<char> {
    let c = self.chars.next()?;
    self.pos += self.len_consumed();
    Some(c)
  }

  pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    while predicate(self.peek()) && !self.is_eof() {
      self.chars.next();
    }
    self.pos += self.len_consumed()
  }

  pub(crate) fn is_eof(&self) -> bool {
    self.chars.as_str().is_empty()
  }

  pub(crate) fn len_consumed(&self) -> usize {
    self.len - self.pos - self.chars.as_str().len()
  } */
}

/* impl<'a> Cursor<'a> {
  pub(crate) fn next_non_whitespace(&mut self) -> char {
    self.next_while(|c| c.is_whitespace());
    self.next_char()
  }
} */
