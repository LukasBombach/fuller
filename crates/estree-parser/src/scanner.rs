use crate::ecma_charset::EcmaCharset;

use std::iter::Peekable;
use std::str::Chars;
use std::string::String;

// todo avoid heap allocation with String for performance
#[derive(Debug, PartialEq)]
pub enum Token {
  Identifier(String),
  Literal(String),
  VariableDeclaration(String),
  Unknown(String),
  Number(u32),
  Semicolon,
  EqalOperator,
  LineBreak,
}

struct Location {
  pub start: Position,
  pub end: Position,
}

struct Position {
  pub line: u16,
  pub column: u16,
}

impl Position {
  pub fn zero() -> Self {
    Self { line: 0, column: 0 }
  }

  pub fn advance(&mut self) {
    self.column += 1;
  }

  pub fn newline(&mut self) {
    self.line += 1;
    self.column = 0;
  }
}

// todo avoid heap allocation with String for performance
pub struct Scanner<'a> {
  input: Peekable<Chars<'a>>,
  loc: Position,
}

impl<'a> Scanner<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input: input.chars().peekable(),
      loc: Position::zero(),
    }
  }

  fn next_input(&mut self) -> Option<char> {
    self.loc.advance();
    self.input.next()
  }
}

// todo maybe match the first char against a narrowed down list of keywords so we know what we got a little earlier
impl<'a> Iterator for Scanner<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.next_input() {
        Some('=') => return Some(Token::EqalOperator),
        Some(';') => return Some(Token::Semicolon),
        Some('\n') => {
          self.loc.newline();
          return Some(Token::LineBreak);
        }
        Some('\r') => continue,
        Some(c) if c.is_whitespace() => continue,
        Some(c) if c.is_id_start() => return self.identifier(&c),
        Some(c) if c.is_quote() => return self.literal(&c),
        Some(c) if c.is_number() => return self.number(&c),
        Some(c) => return Some(Token::Unknown(c.to_string())),
        None => return None,
      }
    }
  }
}

impl<'a> Scanner<'a> {
  // todo avoid heap allocation with String for performance
  fn literal(&mut self, quote_symbol: &char) -> Option<Token> {
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
        Some(c) if c == *quote_symbol => break,
        Some(c) => value.push(c),
        None => break,
      }
    }
    Some(Token::Literal(value))
  }

  // todo avoid heap allocation with String for performance
  fn identifier(&mut self, first_char: &char) -> Option<Token> {
    let mut value = first_char.to_string();
    let continued_value = self
      .input
      .by_ref()
      .take_while(|c| c.is_id_continue())
      .collect::<String>();
    value.push_str(&continued_value);

    match value.as_ref() {
      "const" => Some(Token::VariableDeclaration(value)),
      _ => Some(Token::Identifier(value)),
    }
  }

  // todo avoid heap allocation with String for performance
  fn number(&mut self, first_char: &char) -> Option<Token> {
    let mut value = first_char.to_string();
    let continued_value = self
      .input
      .by_ref()
      .take_while(|c| c.is_number())
      .collect::<String>();
    value.push_str(&continued_value);
    Some(Token::Number(value.parse::<u32>().unwrap()))
  }
}
