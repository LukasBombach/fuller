extern crate ucd;
use std::str::Chars;
use ucd::Codepoint;

#[derive(Debug, PartialEq)]
enum Inner {
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
}

fn get_inner_ast_type(mut code: Chars) -> Inner {
    match code.next() {
        Some('{') => Inner::Block,
        Some(';') => Inner::EmptyStatement,
        Some('d') => match code.next() {
            Some('o') => match code.next() {
                Some(c) if c.is_id_continue() => Inner::Identifier,
                Some(_) => Inner::DoWhileStatement,
                None => Inner::Eos,
            },
            _ => Inner::Identifier,
        },
        Some('i') => match code.next() {
            Some('f') => Inner::IfStatement,
            _ => Inner::Identifier,
        },
        Some('f') => {
            if code.take(2).collect::<Vec<char>>() == ['o', 'r'] {
                Inner::ForLoop
            } else {
                Inner::Identifier
            }
        }
        _ => Inner::Identifier,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block() {
        assert_eq!(get_inner_ast_type("{".chars()), Inner::Block);
    }
    #[test]
    fn test_empty_statement() {
        assert_eq!(get_inner_ast_type(";".chars()), Inner::EmptyStatement);
    }
    #[test]
    fn test_if_statement() {
        assert_eq!(get_inner_ast_type("if".chars()), Inner::IfStatement);
    }
    #[test]
    fn test_for_loop() {
        assert_eq!(get_inner_ast_type("for".chars()), Inner::ForLoop);
    }
}

fn main() {
    get_inner_ast_type(";".chars());
}
