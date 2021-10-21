use std::str;

/* pub struct Node<'a> {
  pub r#type: &'a str,
  pub loc: Option<SourceLocation<'a>>,
}



 */

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

pub fn parse(source_code: &str) -> Program {
  Program {
    r#type: "Program",
    loc: Some(SourceLocation {
      source: Some(""),
      start: Position { line: 0, column: 0 },
      end: Position { line: 0, column: 0 },
    }),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_program() {
    assert_eq!(
      parse(""),
      Program {
        r#type: "Program",
        loc: Some(SourceLocation {
          source: Some(""),
          start: Position { line: 0, column: 0 },
          end: Position { line: 0, column: 0 },
        })
      }
    );
  }

  // #[test]
  // fn test_parse() {
  //   let source_code = "let myvar = \"value\";";
  //   assert_eq!(parse(source_code), 3);
  // }
}
