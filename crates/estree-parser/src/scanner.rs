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
  Const,
  Semicolon,
  EqalOperator,
  LineBreak,
}

// todo avoid heap allocation with String for performance
pub struct Scanner<'a> {
  input: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input: input.chars().peekable(),
    }
  }
}

// todo maybe match the first char against a narrowed down list of keywords so we know what we got a little earlier
impl<'a> Iterator for Scanner<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.input.next() {
        Some('=') => return Some(Token::EqalOperator),
        Some(';') => return Some(Token::Semicolon),
        Some('\n') => return Some(Token::LineBreak),
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

/**
 * Tokens
 */
impl<'a> Scanner<'a> {
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

  fn identifier(&mut self, first_char: &char) -> Option<Token> {
    let mut value = first_char.to_string();
    let continued_value = self
      .input
      .by_ref()
      .take_while(|c| c.is_id_continue())
      .collect::<String>();
    value.push_str(&continued_value);

    match value.as_ref() {
      "const" => Some(Token::Const),
      _ => Some(Token::Identifier(value)),
    }
  }

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
