use crate::ast::Position;
use crate::ast::Program;
use crate::ast::SourceLocation;
use crate::state::program;

/*
 * ** MASTERPLAN **
 *
 * - Implement an Interator for the sourcecode
 * - each next goes through a state machine
 *
 * _or maybe better_
 *
 * - implement statemachine that creates a tree (AST) while walking the code via recursion (sort of)
 *
 *
 * _anyhow_
 *
 * - each state has its strategy of finding the current token as quickly as possible
 * - take min characters, decide what to do next based on the possibilites of each state
 *
 * _godmode optimization_
 *
 * - instead of handcrafting each optimized strategy, use metaprogramming to implement the code for each state's possibilites
 *
 * ** STRATEGIES INCLUDE **
 *
 *  - take the first character and choose / rule out possibilites
 *  - only possibilites with this letter can be considered further
 *  - take n characters based on what can be ruled out with these characters (sometimes has to be one, sometimes it can be more than one)
 *  - repeat / move on to the next state
 */

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
