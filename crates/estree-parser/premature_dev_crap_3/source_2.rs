use std::str::Chars;

struct Source<'src> {
  pub source: Chars<'src>,
  pub line: usize,
  pub column: usize,
}

impl<'src> Source<'src> {
  pub fn new(source: &'src str) -> Self {
    Self {
      source: source.chars(),
      line: 0,
      column: 0,
    }
  }
}

impl<'a> Iterator for Source<'a> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    self.column += 1;
    self.source.next()
  }
}
