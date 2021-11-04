use crate::location::Location;
use std::iter::Enumerate;
use std::str::Chars;

pub struct Source<'src> {
  input: &'src str,
  chars: Enumerate<Chars<'src>>,
  loc: Location,
}

impl<'src> Source<'src> {
  pub fn new(input: &'src str) -> Self {
    println!("\nInput `{}`\n", input);
    Self {
      input,
      chars: input.chars().enumerate(),
      loc: Location { line: 0, column: 0 },
    }
  }
  pub fn slice(&self, start: usize, end: usize) -> &'src str {
    &self.input[start..end]
  }
}

impl<'src> Iterator for Source<'src> {
  type Item = (usize, char);

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(n) = self.chars.next() {
      match n {
        (_, '\r') => {}
        (_, '\n') => self.loc.newline(),
        (_, _) => self.loc.char(),
      }
      return Some(n);
    }
    None
  }
}
