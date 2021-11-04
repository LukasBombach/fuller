use crate::source::Source;
use crate::token::Token;
use crate::token::Value;

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
        (_, ' ' | '\n' | '\r' | '\t') => {}
        (i, 'a'..='z' | 'A'..='Z' | '_' | '$') => return self.identifier(i),
        (i, c) => println!("pos: {} val: `{}`", i, c),
      }
    }
    None
  }
}

impl<'src> Scanner<'src> {
  #[inline]
  fn identifier(&mut self, start_pos: usize) -> Option<Token<'src>> {
    let start = self.source.from_pos(start_pos);

    let end_pos = match self.source.find_next_index_exclusive(|c| *c == ' ') {
      Some(p) => p,
      None => self.source.len(),
    };

    let str = self.source.slice(start_pos, end_pos);
    let end = self.source.from_pos(end_pos);
    return Some(Token::Identifier(Value { str, start, end }));
  }
}
