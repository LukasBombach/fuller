use std::ops::Range;

pub struct Scanner<'input> {
  pub input: &'input str,
  pub len: usize,
  pos: usize,
  loc: Location,
}

impl<'input> Scanner<'input> {
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

impl<'input> Iterator for Scanner<'input> {
  type Item = &'input str;

  fn next(&mut self) -> Option<Self::Item> {
    self.next_word()
  }
}

impl<'input> Scanner<'input> {
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
    while self.get_at(idx) != Some(" ") {
      idx += 1;
    }
    idx
  }

  fn next_non_whitespace_idx(&self, mut idx: usize) -> usize {
    while self.get_at(idx) == Some(" ") {
      idx += 1;
    }
    idx
  }
}

impl<'input> Scanner<'input> {
  fn get_at(&self, idx: usize) -> Option<&'input str> {
    self.input.get(Range {
      start: idx,
      end: idx + 1,
    })
  }
}
