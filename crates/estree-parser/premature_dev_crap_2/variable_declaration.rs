extern crate ucd;
use std::str;
use std::str::Chars;
use std::vec::Vec;
use ucd::Codepoint;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Inner {
  Let,
  Const,
  Identifier,
  Eos,
  Unexpected(char),
}

/*
 * let
 * const
 *        Identifier
 *                    =
 *                       Assignment
 */

struct VariableDeclaration<'a> {
  r#type: &'a str,
  start: u16,
  end: u16,
  kind: &'a str,
  declarations: Vec<VariableDeclarator<'a>>,
}

struct VariableDeclarator<'a> {
  r#type: &'a str,
  start: u16,
  end: u16,
  kind: &'a str,
  id: Identifier<'a>,
  init: Literal<'a>,
}

struct Identifier<'a> {
  r#type: &'a str,
  start: u16,
  end: u16,
  name: &'a str,
}

struct Literal<'a> {
  r#type: &'a str,
  start: u16,
  end: u16,
  name: &'a str,
}

pub fn get_ast(mut code: Chars) -> VariableDeclaration {
  let r#type = "VariableDeclarator";
  let kind = match code.next() {
    Some('l') => {
      code.skip(2);
      "let"
    }
    Some('c') => {
      code.skip(3);
      "const"
    }
    Some(c) => panic!("unexpected character {}", c),
    None => panic!("end of stream"),
  };

  let mut code = code.skip_while(|c| !c.is_whitespace());

  let identifier = {
    let start_character = if let Some(c) = code.next() {
      if !c.is_id_start() {
        panic!("illegal character {} at the beginning of an identifier", c);
      } else {
        c
      }
    } else {
      panic!("unexpected eos");
    };

    let continued_characters = code
      .take_while(|c| c.is_id_continue())
      .collect::<String>()
      .as_str();

    //let id = continued_characters
  };
}

#[cfg(test)]
mod tests {
  use super::*;
}
