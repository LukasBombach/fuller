// use crate::scanner::Location;

#[derive(Debug)]
pub enum Token<'a> {
  Eq,
  Semicolon,
  Keyword,
  Newline,
  Const,
  Unknown(&'a str),
  Identifier(&'a str),
  Literal(&'a str),
  Number(&'a str),
}
// Unknown((&'a str, Location)),
// Identifier((&'a str, Location)),
// Literal((&'a str, Location)),
// Number((&'a str, Location)),
