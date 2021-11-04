#[derive(Debug)]
pub enum Token {
  Eq,
  Semicolon,
  Keyword,
  Newline,
  Const,
  Unknown,
  Identifier(String),
  Literal(String),
  Number(String),
}
