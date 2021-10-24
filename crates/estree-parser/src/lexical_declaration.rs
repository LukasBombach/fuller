extern crate ucd;
use std::str::Chars;
use ucd::Codepoint;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Inner {
  Let
  Const
  Identifier
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

pub fn get_ast(mut code: Chars) -> Inner {
  match code.next() {
    Some('l') => {
      code.skip(2);
    }
    Some('c') => {
      code.skip(3);

    }
    Some(c) => Inner::Unexpected
    None => Inner::Eos
  }
}

#[cfg(test)]
mod tests {
  use super::*;


}
