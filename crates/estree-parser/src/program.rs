extern crate ucd;
use std::str::Chars;
use ucd::Codepoint;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Inner {
  Block,
  EmptyStatement,
  DoWhileStatement,
  IfStatement,
  ForLoop,
  LexicalDeclaration,
  TryStatement,
  WithStatement,
  BreakStatement,
  ClassDeclaration,
  ThrowStatement,
  WhileLoop,
  ReturnStatement,
  ContinueStatement,
  DebuggerStatement,
  FunctionDeclaration,
  AsyncKeyword,
  Identifier,
  Eos,
  Unexpected(char),
}

pub fn get_ast(mut code: Chars) -> Inner {
  match code.next() {
    Some('{') => Inner::Block,
    Some(';') => Inner::EmptyStatement,
    Some('d') => match code.next() {
      Some('o') => match code.next() {
        Some(c) if c.is_id_continue() => Inner::Identifier,
        Some(_) => Inner::DoWhileStatement,
        None => Inner::Eos,
      },
      Some(c) if c.is_id_continue() => Inner::Identifier,
      Some(c) => Inner::Unexpected(c),
      None => Inner::Eos,
    },
    Some('i') => match code.next() {
      Some('f') => match code.next() {
        Some(c) if c.is_id_continue() => Inner::Identifier,
        Some(_) => Inner::IfStatement,
        None => Inner::Eos,
      },
      Some(c) => Inner::Unexpected(c),
      None => Inner::Eos,
    },
    Some('f') => match code.next() {
      Some('o') => match code.next() {
        Some('r') => match code.next() {
          Some(c) if c.is_id_continue() => Inner::Identifier,
          Some(_) => Inner::ForLoop,
          None => Inner::Eos,
        },
        Some(c) => Inner::Unexpected(c),
        None => Inner::Eos,
      },
      Some(c) if c.is_id_continue() => Inner::Identifier,
      Some(c) => Inner::Unexpected(c),
      None => Inner::Eos,
    },
    Some('l') => match code.next() {
      Some('e') => match code.next() {
        Some('t') => match code.next() {
          Some(c) if c.is_id_continue() => Inner::Identifier,
          Some(_) => Inner::LexicalDeclaration,
          None => Inner::Eos,
        },
        Some(c) => Inner::Unexpected(c),
        None => Inner::Eos,
      },
      Some(c) if c.is_id_continue() => Inner::Identifier,
      Some(c) => Inner::Unexpected(c),
      None => Inner::Eos,
    },
    Some('v') => match code.next() {
      Some('a') => match code.next() {
        Some('r') => match code.next() {
          Some(c) if c.is_id_continue() => Inner::Identifier,
          Some(_) => Inner::LexicalDeclaration,
          None => Inner::Eos,
        },
        Some(c) => Inner::Unexpected(c),
        None => Inner::Eos,
      },
      Some(c) if c.is_id_continue() => Inner::Identifier,
      Some(c) => Inner::Unexpected(c),
      None => Inner::Eos,
    },
    Some('c') => match code.next() {
      Some('o') => match code.next() {
        Some('n') => match code.next() {
          Some('s') => match code.next() {
            Some('t') => match code.next() {
              Some(c) if c.is_id_continue() => Inner::Identifier,
              Some(_) => Inner::LexicalDeclaration,
              None => Inner::Eos,
            },
            Some(c) => Inner::Unexpected(c),
            None => Inner::Eos,
          },
          Some(c) if c.is_id_continue() => Inner::Identifier,
          Some(c) => Inner::Unexpected(c),
          None => Inner::Eos,
        },

        Some(c) if c.is_id_continue() => Inner::Identifier,
        Some(c) => Inner::Unexpected(c),
        None => Inner::Eos,
      },
      Some(c) if c.is_id_continue() => Inner::Identifier,
      Some(c) => Inner::Unexpected(c),
      None => Inner::Eos,
    },
    Some('w') => match code.next() {
      Some('h') => match code.next() {
        Some('i') => match code.next() {
          Some('l') => match code.next() {
            Some('e') => match code.next() {
              Some(c) if c.is_id_continue() => Inner::Identifier,
              Some(_) => Inner::WhileLoop,
              None => Inner::Eos,
            },
            Some(c) => Inner::Unexpected(c),
            None => Inner::Eos,
          },
          Some(c) if c.is_id_continue() => Inner::Identifier,
          Some(c) => Inner::Unexpected(c),
          None => Inner::Eos,
        },

        Some(c) if c.is_id_continue() => Inner::Identifier,
        Some(c) => Inner::Unexpected(c),
        None => Inner::Eos,
      },
      Some(c) if c.is_id_continue() => Inner::Identifier,
      Some(c) => Inner::Unexpected(c),
      None => Inner::Eos,
    },
    Some(_) => Inner::Identifier,
    None => Inner::Eos,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_block() {
    assert_eq!(get_ast("{".chars()), Inner::Block);
  }
  #[test]
  fn test_empty_statement() {
    assert_eq!(get_ast(";".chars()), Inner::EmptyStatement);
  }
  #[test]
  fn test_do_while_statement() {
    assert_eq!(get_ast("do ".chars()), Inner::DoWhileStatement);
  }
  #[test]
  fn test_if_statement() {
    assert_eq!(get_ast("if ".chars()), Inner::IfStatement);
  }
  #[test]
  fn test_for_loop() {
    assert_eq!(get_ast("for ".chars()), Inner::ForLoop);
  }
  #[test]
  fn test_let_lexical_declaration() {
    assert_eq!(get_ast("let ".chars()), Inner::LexicalDeclaration);
  }
  #[test]
  fn test_var_lexical_declaration() {
    assert_eq!(get_ast("var ".chars()), Inner::LexicalDeclaration);
  }
  #[test]
  fn test_const_lexical_declaration() {
    assert_eq!(get_ast("const ".chars()), Inner::LexicalDeclaration);
  }
  #[test]
  fn test_while_loop_with_space() {
    assert_eq!(get_ast("while ".chars()), Inner::WhileLoop);
  }
  #[test]
  fn test_while_loop_with_bracket() {
    assert_eq!(get_ast("while(".chars()), Inner::WhileLoop);
  }
  #[test]
  fn test_beginning_of_keyword_but_continued_as_identifier() {
    assert_eq!(get_ast("doX".chars()), Inner::Identifier);
    assert_eq!(get_ast("ifX".chars()), Inner::Identifier);
    assert_eq!(get_ast("forX".chars()), Inner::Identifier);
    assert_eq!(get_ast("letX".chars()), Inner::Identifier);
    assert_eq!(get_ast("varX".chars()), Inner::Identifier);
    assert_eq!(get_ast("constX".chars()), Inner::Identifier);
    assert_eq!(get_ast("whileX".chars()), Inner::Identifier);
  }
}
