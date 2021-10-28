use crate::ecma_charset::EcmaCharset;
use crate::token::Position;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::Value;

use std::iter::Peekable;
use std::str::Chars;
use std::string::String;

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
}

impl<'a> Iterator for Scanner<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.input.next() {
        Some('=') => return self.token(TokenType::Punctuator, Value::Str(String::from("=")), 1),
        Some(';') => return self.token(TokenType::Punctuator, Value::Str(String::from(";")), 1),
        Some('\n') => {
          self.loc.newline();
          continue;
        }
        Some('\r') => continue,
        Some(c) if c.is_whitespace() => {
          self.loc.advance(1);
          continue;
        }
        Some(c) if c.is_id_start() => return self.identifier(&c),
        Some(c) if c.is_quote() => return self.literal(&c),
        Some(c) if c.is_number() => return self.number(&c),
        Some(c) => panic!("unable to parse {}", c),
        None => return None,
      }
    }
  }
}

impl<'a> Scanner<'a> {
  fn token(&mut self, tokentype: TokenType, value: Value, advance_by: usize) -> Option<Token> {
    let start = self.loc.clone();
    self.loc.advance(advance_by);
    let end = self.loc.clone();
    Some(Token {
      tokentype,
      value,
      start,
      end,
    })
  }

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

    let start = self.loc.clone();
    self.loc.advance(value.len());
    let end = self.loc.clone();

    Some(Token {
      tokentype: TokenType::StringLiteral,
      value: Value::Str(value),
      start,
      end,
    })
  }

  fn identifier(&mut self, first_char: &char) -> Option<Token> {
    let mut value = first_char.to_string();
    let continued_value = self
      .input
      .by_ref()
      .take_while(|c| c.is_id_continue())
      .collect::<String>();
    value.push_str(&continued_value);

    let start = self.loc.clone();
    self.loc.advance(value.len());
    let end = self.loc.clone();

    match value.as_ref() {
      "const" => Some(Token {
        tokentype: TokenType::Keyword,
        value: Value::Str(value),
        start,
        end,
      }),
      _ => Some(Token {
        tokentype: TokenType::Identifier,
        value: Value::Str(value),
        start,
        end,
      }),
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

    let start = self.loc.clone();
    self.loc.advance(value.len());
    let end = self.loc.clone();

    Some(Token {
      tokentype: TokenType::NumericLiteral,
      value: Value::Num(value.parse::<f64>().unwrap()),
      start,
      end,
    })
  }
}
