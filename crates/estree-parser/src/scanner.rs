use crate::source::Source;
use crate::token::Token;

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
