use crate::location::Location;
use std::iter::Enumerate;
use std::str::Chars;

pub struct Source<'src> {
  input: &'src str,
  chars: Enumerate<Chars<'src>>,
  lines: usize,
  last_line: usize,
}

impl<'src> Source<'src> {
  pub fn new(input: &'src str) -> Self {
    println!("\nInput `{}`\n", input);
    Self {
      input,
      chars: input.chars().enumerate(),
      lines: 0,
      last_line: 0,
    }
  }

  #[inline]
  pub fn slice(&self, start: usize, end: usize) -> &'src str {
    &self.input[start..end]
  }

  #[inline]
  pub fn from_pos(&self, pos: usize) -> Location {
    Location(self.lines, pos - self.last_line)
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.input.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.input.is_empty()
  }
}

impl<'src> Iterator for Source<'src> {
  type Item = (usize, char);

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(n) = self.chars.next() {
      if let (i, '\n') = n {
        self.last_line = i;
        self.lines += 1;
      }
      return Some(n);
    }
    None
  }
}

impl<'src> Source<'src> {
  #[inline]
  pub fn find_next_index_exclusive<P>(&mut self, predicate: P) -> Option<usize>
  where
    P: Fn(&char) -> bool,
  {
    if let Some((pos, _)) = self
      .chars
      .by_ref()
      .take_while(|(_, c)| !predicate(c))
      .last()
    {
      return Some(pos + 1);
    }
    None
  }
}
