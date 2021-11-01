use crate::token::Token;
// use std::ops::Range;
// use std::string::String;

pub struct Location {
  pub line: usize,
  pub column: usize,
}

pub struct Scanner<'src> {
  source: &'src str,
  loc: Location,
  len: usize,
  pos: usize,
}

impl<'src> Scanner<'src> {
  pub fn new(source: &'src str) -> Self {
    Self {
      source,
      loc: Location { line: 0, column: 0 },
      len: source.len(),
      pos: 0,
    }
  }
}

impl<'src> Iterator for Scanner<'src> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    /* if let Some(s) = self.source.get(Range {
      start: self.pos,
      end: self.pos + 1,
    }) {
      let c = char::from_str(s).unwrap();
    } */

    if self.pos == self.len {
      return None;
    }

    // todo use pointers, not values
    let next_token = match self.source.as_bytes()[self.pos] as char {
      // ' ' => {}
      '=' => self.token(Token::Eq, 1),
      ';' => self.token(Token::Semicolon, 1),
      '\n' => self.newline(),
      '\r' => self.newline(),
      // c if matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '$') => return self.identifier(c),
      // c if matches!(c, '"' | '\'') => return self.literal(c),
      // c if matches!(c, '0'..='9') => return self.number(c),
      // todo c matches unicode ID_Start
      c => self.token(Token::Unknown(c), 1),
    };

    self.pos += 1;

    next_token
  }
}

impl<'src> Scanner<'src> {
  /*
  // todo avoid heap allocation with String
  // todo take_while takes one too many characters without returning it
  #[inline]
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
  #[inline]
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
  #[inline]
  fn number(&mut self, first_num: char) -> Option<Token> {
    let continued_characters = self
      .source
      .by_ref()
      .take_while(|c| matches!(*c, '1'..='9'))
      .collect::<String>();
    let identifier = format!("{}{}", first_num, continued_characters);
    let len = identifier.len();
    self.token(Token::Identifier(identifier), len)
  } */

  #[inline]
  fn token(&mut self, token: Token, length: usize) -> Option<Token> {
    self.loc.column += length;
    Some(token)
  }

  #[inline]
  fn newline(&mut self) -> Option<Token> {
    self.loc.column = 0;
    Some(Token::Newline)
  }
}
