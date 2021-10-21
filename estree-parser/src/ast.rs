use std::str;

pub struct Identifier<'a> {
  pub location: Option<SourceLocation<'a>>,
  pub name: &'a str,
}

pub struct SourceLocation<'a> {
  pub source: Option<&'a str>;
  pub start: Position,
  pub end: Position,
}

pub struct Position {
  pub line: u16,
  pub column: u16,
}
