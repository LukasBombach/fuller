#[derive(Debug)]
pub enum Token<'a> {
  Eq,
  Semicolon,
  Keyword,
  Newline,
  Const,
  Unknown(char),
  Identifier(&'a str),
  Literal(&'a str),
  Number(&'a str),
}
