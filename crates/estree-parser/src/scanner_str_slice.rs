use crate::token::Token;
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

pub struct Location {
  pub line: usize,
  pub column: usize,
}

impl Location {
  pub fn char(&mut self) {
    self.column += 1;
  }

  pub fn newline(&mut self) {
    self.column = 0;
    self.line += 1;
  }
}

pub struct Scanner<'src> {
  source: Source<'src>,
}

impl<'src> Scanner<'src> {
  pub fn new(input: &'src str) -> Self {
    Self {
      source: Source::new(input),
    }
  }
}

impl<'src> Iterator for Scanner<'src> {
  type Item = Token<'src>;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(n) = self.source.next() {
      match n {
        (_, ' ') => {}
        (i, c) if matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '$') => return self.identifier(i),
        (i, c) => println!("{} {}", i, c),
      }
    }
    None
  }
}

impl<'src> Scanner<'src> {
  #[inline]
  fn identifier(&mut self, start: usize) -> Option<Token<'src>> {
    if let Some((end, _)) = self.source.by_ref().take_while(|(_, c)| *c != ' ').last() {
      let value = self.source.slice(start, end + 1);
      return Some(Token::Identifier(value));
    }
    None
  }
}
