use crate::location::Location;

#[derive(Debug)]
pub enum Token<'a> {
  Eq,
  Semicolon,
  Keyword,
  Newline,
  Const,
  Unknown(Value<'a>),
  Identifier(Value<'a>),
  Literal(Value<'a>),
  Number(Value<'a>),
}

#[derive(Debug)]
pub struct Value<'a> {
  pub str: &'a str,
  pub start: Location,
  pub end: Location,
}
