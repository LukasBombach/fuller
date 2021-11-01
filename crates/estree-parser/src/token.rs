use std::string::String;

#[derive(Debug)]
pub enum Token {
  Eq,
  Semicolon,
  Keyword,
  Newline,
  Const,
  Unknown(char),
  Identifier(String),
  Literal(String),
  Number(String),
}
