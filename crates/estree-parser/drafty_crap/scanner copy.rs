use crate::source::Source;
use crate::token::Token;

pub struct Position {
  pub line: usize,
  pub column: usize,
}

pub struct Scanner<'src> {
  source: Source<'src>,
  position: Position,
}

impl<'src> Scanner<'src> {
  pub fn new(source: Source<'src>) -> Self {
    Self {
      source,
      position: Position { line: 0, column: 0 },
    }
  }
}

impl<'src> Iterator for Scanner<'src> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.source.next_non_whitespace() {
        Some("=") => return self.token(Token::Eq, 1),
        Some(";") => return self.token(Token::Semi, 1),
        Some("\n") => return self.token(Token::Newline, 1),
        Some("\r") => {}
        Some("a"..="z") | Some("A"..="Z") | Some("_") | Some("$") => return self.identifier(&c),
        Some(c) if c.is_quote() => return self.literal(&c),
        Some(c) if c.is_number() => return self.number(&c),
        Some(c) => panic!("unable to parse {}", c),
        None => return None,
      }
    }
  }
}

impl<'src> Scanner<'src> {
  fn token(&mut self, token: Token, length: usize) -> Option<Token> {
    self.position.column += length;
    Some(token)
  }
}
