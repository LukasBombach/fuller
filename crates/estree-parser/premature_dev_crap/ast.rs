use std::str;

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
  pub r#type: &'a str,
  pub loc: Option<SourceLocation<'a>>,
  // todo pub body: [ Directive | Statement ],
}

#[derive(Debug, PartialEq)]
pub struct SourceLocation<'a> {
  pub source: Option<&'a str>,
  pub start: Position,
  pub end: Position,
}

#[derive(Debug, PartialEq)]
pub struct Position {
  pub line: u16,
  pub column: u16,
}
