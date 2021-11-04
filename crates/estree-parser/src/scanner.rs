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
    if let Some((end_pos, _)) = self.source.by_ref().take_while(|(_, c)| *c != ' ').last() {
      let str = self.source.slice(start_pos, end_pos + 1);
      let end = self.source.from_pos(end_pos + 1);
      return Some(Token::Identifier(Value { str, start, end }));
    }
    None
  }
}
