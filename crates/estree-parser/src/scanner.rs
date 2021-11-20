use crate::source::Source;
use crate::token::Pos;
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
        (i, '=') => return self.eq(i),
        (i, ';') => return self.semicolon(i),
        (i, '"') => return self.string_literal(i, '"'),
        (i, '\'') => return self.string_literal(i, '\''),
        // (i, '<') => return self.lt(i),
        (i, 'a'..='z' | 'A'..='Z' | '_' | '$') => return self.identifier(i),
        (i, c) => println!("pos: {} val: `{}`", i, c),
      }
    }
    None
  }
}

impl<'src> Scanner<'src> {
  // todo this does not advance the iterator
  /* #[inline]
  fn lt(&mut self, start_pos: usize) -> Option<Token<'src>> {
    match self.source.peek(start_pos, 1) {
      Some("=") => Some(Token::Lte(self.pos(start_pos, 2))),
      Some("<") => match self.source.peek(start_pos + 1, 1) {
        Some("=") => Some(Token::AssignShl(self.pos(start_pos, 3))),
        _ => Some(Token::Shl(self.pos(start_pos, 2))),
      },
      _ => Some(Token::Lt(self.pos(start_pos, 1))),
    }
  } */

  #[inline]
  fn eq(&mut self, start_pos: usize) -> Option<Token<'src>> {
    Some(Token::Eq(self.pos(start_pos, 1)))
  }

  #[inline]
  fn semicolon(&mut self, start_pos: usize) -> Option<Token<'src>> {
    Some(Token::Semicolon(self.pos(start_pos, 1)))
  }

  #[inline]
  fn pos(&mut self, start_pos: usize, len: usize) -> Pos {
    let start = self.source.from_pos(start_pos);
    let end = self.source.from_pos(start_pos + len);
    Pos { start, end }
  }

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

  // todo do not copy a char but reference the input str
  // todo end_pos code is really bad
  // todo if I implement the iterator myself keeping track of the idx and the len I can just peek and then advance for the given length
  #[inline]
  fn string_literal(&mut self, start_pos: usize, delimiter: char) -> Option<Token<'src>> {
    let start = self.source.from_pos(start_pos);

    let end_pos: usize = {
      let p: usize;
      loop {
        match self.source.next() {
          Some((_, '\\')) => {
            self.source.next();
          }
          Some((i, c)) if c == delimiter => {
            p = i + 1;
            break;
          }
          _ => continue,
        }
      }
      p
    };

    let str = self.source.slice(start_pos, end_pos);
    let end = self.source.from_pos(end_pos);
    return Some(Token::Literal(Value { str, start, end }));
  }
}
