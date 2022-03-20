use std::str::Chars;

pub(crate) const EOF_CHAR: char = '\0';

/// Peekable iterator over a char sequence.
pub(crate) struct Cursor<'a> {
  len: usize,
  chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
  pub(crate) fn new(input: &'a str) -> Cursor<'a> {
    Cursor {
      len: input.len(),
      chars: input.chars(),
    }
  }

  pub(crate) fn peek(&self) -> char {
    self.chars.clone().next().unwrap_or(EOF_CHAR)
  }

  pub(crate) fn first(&mut self) -> Option<char> {
    let c = self.chars.next()?;
    Some(c)
  }

  pub(crate) fn is_eof(&self) -> bool {
    self.chars.as_str().is_empty()
  }

  pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    while predicate(self.peek()) && !self.is_eof() {
      self.first();
    }
  }
}
