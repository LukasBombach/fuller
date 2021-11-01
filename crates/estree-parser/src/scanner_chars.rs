use crate::token_string::Token;
use std::str::Chars;

pub struct Location {
  pub line: usize,
  pub column: usize,
}

pub struct Scanner<'src> {
  source: Chars<'src>,
  loc: Location,
  pos: usize,
}

impl<'src> Scanner<'src> {
  pub fn new(source: &'src str) -> Self {
    Self {
      source: source.chars(),
      loc: Location { line: 0, column: 0 },
      pos: 0,
    }
  }
}

impl<'src> Iterator for Scanner<'src> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    match self.source.find(|c| *c != ' ') {
      Some('=') => self.token(Token::Eq, 1),
      Some(';') => self.token(Token::Semicolon, 1),
      Some('\n') => self.newline(),
      Some('\r') => self.newline(),
      Some(c) if matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '$') => self.identifier(),
      Some(c) => self.token(Token::Unknown(c), 1),
      None => None,
    }
  }
}

impl<'src> Scanner<'src> {
  #[inline]
  fn identifier(&mut self) -> Option<Token> {
    let tail = self
      .source
      .by_ref()
      .take_while(|c| *c != ' ')
      .collect::<String>();
    let len = tail.len();
    self.token(Token::Identifier(tail), len)
  }

  #[inline]
  fn token(&mut self, token: Token, len: usize) -> Option<Token> {
    self.pos += len;
    self.loc.column += len;
    Some(token)
  }

  #[inline]
  fn newline(&mut self) -> Option<Token> {
    self.loc.line += 1;
    self.loc.column = 0;
    self.pos += 1;
    Some(Token::Newline)
  }
}
