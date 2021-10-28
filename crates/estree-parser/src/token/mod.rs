pub mod punctuators;
pub mod value;

pub use crate::token::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
  BooleanLiteral,
  Identifier,
  Keyword,
  NullLiteral,
  NumericLiteral,
  StringLiteral,
  Punctuator,
  RegularExpression,
  Template,
  Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub tokentype: TokenType,
  pub value: Value,
  pub start: Position,
  pub end: Position,
}

#[derive(Debug, Clone)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}

impl Position {
  pub fn zero() -> Self {
    Self { line: 0, column: 0 }
  }

  pub fn advance(&mut self, num_chars: usize) {
    self.column += num_chars;
  }

  pub fn newline(&mut self) {
    self.line += 1;
    self.column = 0;
  }
}
