use std::ops::Range;

struct Source<'src> {
  pub source: &'src str,
  pub pos: usize,
  pub line: usize,
  pub column: usize,
}

impl<'src> Source<'src> {
  pub fn new(source: &'src str) -> Self {
    Self {
      source,
      pos: 0,
      line: 0,
      column: 0,
    }
  }
}

impl<'a> Iterator for Source<'a> {
  type Item = &'a str;

  fn next(&mut self) -> Option<Self::Item> {
    let item = self.source.get(Range {
      start: self.pos,
      end: self.pos + 1,
    });
    self.pos += 1;
    item
  }
}
