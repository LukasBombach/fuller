extern crate ucd;
use std::iter::Peekable;
use std::str::Chars;
use std::string::String;
use ucd::Codepoint;

// todo avoid heap allocation with String for performance
#[derive(Debug, PartialEq)]
pub enum Token {
  Literal(String),
  VariableDeclaration(String),
  LineBreak,
  Eos,
}

// todo avoid heap allocation with String for performance
pub struct Scanner<'a> {
  input: Peekable<Chars<'a>>,
  current: String,
}

impl<'a> Scanner<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input: input.chars().peekable(),
      current: String::new(),
    }
  }
}

// todo maybe match the first char against a narrowed down list of keywords so we know what we got a little earlier
impl<'a> Iterator for Scanner<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.input.next() {
        Some(c) if self.is_whitespace(c) => continue,
        Some(c) if self.is_id_start(c) => return self.identifier(c),
        Some(c) if self.is_quote(c) => return self.literal(c),
        Some('\n') => return Some(Token::LineBreak),
        Some('\r') => continue,
        None => return Some(Token::Eos),
      }
    }
  }
}

/**
 * Tokens
 */
impl<'a> Scanner<'a> {
  fn literal(&mut self, quote_symbol: char) -> Option<Token> {
    let mut value = String::new();
    loop {
      match self.input.next() {
        Some('\\') => {
          value.push('\\');
          match self.input.next() {
            Some(c) => value.push(c),
            None => break,
          }
        }
        Some(c) if c == quote_symbol => break,
        Some(c) => value.push(c),
        None => break,
      }
    }
    Some(Token::Literal(value))
  }

  fn identifier(&mut self, first_char: char) -> Option<Token> {}
}

/**
 * Matchers
 */
impl<'a> Scanner<'a> {
  fn is_id_start(&self, c: char) -> bool {
    match c {
      '$' | '_' | 'A'..='Z' | 'a'..='z' => true,
      _ => c.is_id_start(),
    }
  }

  fn is_whitespace(&self, c: char) -> bool {
    match c {
      '\u{0009}' | '\u{000B}' | '\u{000C}' | '\u{0020}' | '\u{00A0}' | '\u{FEFF}' | '\u{1680}'
      | '\u{2000}' | '\u{2001}' | '\u{2002}' | '\u{2003}' | '\u{2004}' | '\u{2005}'
      | '\u{2006}' | '\u{2007}' | '\u{2008}' | '\u{2009}' | '\u{200A}' | '\u{202F}'
      | '\u{205F}' | '\u{3000}' => true,
      _ => false,
    }
  }

  fn is_quote(&self, c: char) -> bool {
    match c {
      '"' | '\'' => true,
      _ => false,
    }
  }
}
