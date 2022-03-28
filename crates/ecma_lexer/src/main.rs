use std::str::Chars;
use TokenKind::*;

#[derive(Debug)]
enum TokenKind {
    Const,
    Ident,
    Eof,
    Unexpected,
}

/* enum Mode {
    Program,
    KeywordOrIdent,
    Ident,
    Whitespace,
} */

pub trait Mode {
    fn test(c: &char) -> ();
}

pub struct Program;
impl Mode for Program {
    fn test(c: &char) -> () {}
}

pub struct Lexer<'a, M>
where
    M: Mode,
{
    chars: Chars<'a>,
    tokens: Vec<TokenKind>,
    mode: M,
}

impl<'a> Lexer<'a, Program> {
    pub fn new(input: &'a str) -> Lexer<'a, Program> {
        Lexer {
            chars: input.chars(),
            tokens: Vec::new(),
            mode: Program {},
        }
    }

    pub fn lex(&mut self) -> () {
        loop {
            let c = self.chars.next().unwrap_or_default();
            self.mode
        }
    }

    fn keyword_or_ident(&mut self) -> () {}

    fn token(&mut self, token_kind: TokenKind) -> () {
        self.tokens.push(token_kind);
    }
}

fn main() {
    let input = "const conrad";
}
