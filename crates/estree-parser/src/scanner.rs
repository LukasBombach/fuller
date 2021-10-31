use crate::token::Token;
use std::str::Chars;
use std::string::String;

pub struct Position {
  pub line: usize,
  pub column: usize,
}

pub struct Scanner<'src> {
  source: Chars<'src>,
  position: Position,
}

impl<'src> Scanner<'src> {
  pub fn new(source: Chars<'src>) -> Self {
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
      match self.source.next() {
        Some(' ') => {}
        Some('=') => return self.token(Token::Eq, 1),
        Some(';') => return self.token(Token::Semicolon, 1),
        Some('\n') => return self.newline(),
        Some('\r') => {}
        Some(c) if matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '$') => return self.identifier(c),
        Some(c) if matches!(c, '"' | '\'') => return self.literal(c),
        Some(c) if matches!(c, '0'..='9') => return self.number(c),
        // todo c matches unicode ID_Start
        Some(c) => panic!("unable to parse `{}`", c),
        None => return None,
      }
    }
  }
}

impl<'src> Scanner<'src> {
  // todo avoid heap allocation with String
  // todo take_while takes one too many characters without returning it
  fn identifier(&mut self, first_char: char) -> Option<Token> {
    let continued_characters = self
      .source
      .by_ref()
      .take_while(|c| *c != ' ')
      .collect::<String>();
    let identifier = format!("{}{}", first_char, continued_characters);
    let len = identifier.len();
    self.token(Token::Identifier(identifier), len)
  }

  // todo avoid heap allocation with String
  // todo take_while takes one too many characters without returning it
  fn literal(&mut self, quot: char) -> Option<Token> {
    let continued_characters = self
      .source
      .by_ref()
      .take_while(|c| *c != quot)
      .collect::<String>();
    let identifier = format!("{}{}{}", quot, continued_characters, quot);
    let len = identifier.len();
    self.token(Token::Literal(identifier), len)
  }

  // todo avoid heap allocation with String
  // todo take_while takes one too many characters without returning it
  fn number(&mut self, first_num: char) -> Option<Token> {
    let continued_characters = self
      .source
      .by_ref()
      .take_while(|c| matches!(*c, '1'..='9'))
      .collect::<String>();
    let identifier = format!("{}{}", first_num, continued_characters);
    let len = identifier.len();
    self.token(Token::Identifier(identifier), len)
  }

  fn token(&mut self, token: Token, length: usize) -> Option<Token> {
    self.position.column += length;
    Some(token)
  }

  fn newline(&mut self) -> Option<Token> {
    self.position.column = 0;
    Some(Token::Newline)
  }
}
