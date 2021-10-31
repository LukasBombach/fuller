// use std::ops::Add;
use std::ops::Range;

/* pub trait SourceRange<'a, Idx> {
  fn get_one(idx: Idx) -> Self;
}

impl<'a, Idx> SourceRange<'a, Idx> for Range<Idx>
where
  Idx: Add,
{
  fn get_one(idx: Idx) -> Self {
    Range {
      start: idx,
      end: idx + 1,
    }
  }
} */

pub struct Source<'input> {
  pub input: &'input str,
  pub len: usize,
  pos: usize,
  loc: Location,
}

impl<'input> Source<'input> {
  pub fn new(input: &'input str) -> Self {
    Self {
      input,
      len: input.len(),
      pos: 0,
      loc: Location { line: 0, column: 0 },
    }
  }
}

#[derive(Debug, Clone)]
pub struct Location {
  pub line: usize,
  pub column: usize,
}

/* impl<'input> Iterator for Source<'input> {
  type Item = &'input str;

  fn next(&mut self) -> Option<Self::Item> {
    self.next_word()
  }
} */

impl<'input> Source<'input> {
  pub fn next_non_whitespace(&mut self) -> Option<&'input str> {
    self.pos = self.next_non_whitespace_idx(self.pos);
    self.input.get(Range {
      start: self.pos,
      end: self.pos + 1,
    })
  }

  fn next_word(&mut self) -> Option<&'input str> {
    if self.pos == self.len {
      return None;
    }
    let start = self.next_non_whitespace_idx(self.pos);
    let end = self.next_whitespace_idx(start);
    self.pos = end;
    Some(unsafe { self.input.get_unchecked(Range { start, end }) })
  }

  fn next_whitespace_idx(&self, mut idx: usize) -> usize {
    loop {
      match self.get_at(idx) {
        Some(" ") => break,
        Some(_) => idx += 1,
        None => break,
      }
    }
    idx
  }

  fn next_non_whitespace_idx(&self, mut idx: usize) -> usize {
    loop {
      match self.get_at(idx) {
        Some(" ") => idx += 1,
        Some(_) => break,
        None => break,
      }
    }
    idx
  }
}

/**
 *   todo extend range to span one character and just use  self.input.get
 * */

impl<'input> Source<'input> {
  fn get_at(&self, idx: usize) -> Option<&'input str> {
    self.input.get(Range {
      start: idx,
      end: idx + 1,
    })
  }
}
