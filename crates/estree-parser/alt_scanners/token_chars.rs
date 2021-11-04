use std::str::Chars;

#[derive(Debug)]
pub enum Token<'a> {
  Eq,
  Semicolon,
  Keyword,
  Newline,
  Const,
  Unknown(char),
  Identifier(Chars<'a>),
  Literal(Chars<'a>),
  Number(Chars<'a>),
}
