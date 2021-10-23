/* extern crate unicode_segmentation;

use crate::ast::Position;
use crate::ast::Program;
use crate::ast::SourceLocation;
use unicode_segmentation::UnicodeSegmentation;

pub fn parse(source_code: &str) -> Program {
  let graphemes = UnicodeSegmentation::graphemes(source_code, true);

  loop {
    match graphemes.next() {
      Some(x) => {
        println!("{}", x);
      }
      None => break,
    }
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
 */
