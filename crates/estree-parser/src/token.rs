use std::string::String;

#[derive(Debug)]
pub enum Token {
  Eq,
  Semicolon,
  Keyword,
  Newline,
  Const,
  Identifier(String),
  Literal(String),
  Number(String),
}
