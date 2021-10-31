use std::string::String;

pub enum Token {
  Eq,
  Semicolon,
  Keyword,
  Newline,
  Const,
  Identifier(String),
}
