/*
use std::str::Chars;
use TokenKind::*;

mod matcher;

#[derive(Debug)]
enum TokenKind {
    CONST,
    IDENT,
    EOF,
    UNEXPECTED,
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
    tokens: Vec<TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
          chars: input.chars(),
          tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        self.program()
    }

    pub fn program(&mut self) {
        loop {
            match self.next() {
                'c' => match self.next() {
                    'o' => match self.next() {
                        'n' => match self.next() {
                            's' => match self.next() {
                                't' => self.token(CONST),
                                _ => self.token(IDENT),
                            },
                            _ => self.token(IDENT),
                        },
                        _ => self.token(IDENT),
                    },
                    _ => self.token(IDENT),
                }
                '\x00' => {
                    self.token(EOF);
                    break;
                }
                _ => self.token(UNEXPECTED)
            }

        }
    }

    fn next(&mut self) -> char {
        self.chars.next().unwrap_or_default()
    }


    fn id_continue(&mut self) -> () {
        loop {
            let c = self.next();

            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {},
                ' ' | '\t' | '\n' | '\r' =>  {
                    self.token(IDENT);
                    break;
                },
                _ => self.token(UNEXPECTED)
            }

        }


    }

    fn token(&mut self, token_kind:TokenKind) -> () {
        self.tokens.push(token_kind);
    }

    fn print_tokens(self) -> () {
        println!("{:#?}", self.tokens);
    }
}

fn main() {
    let input = "const conrad";
    let mut lexer = Lexer::new(input);

    lexer.lex();
    lexer.print_tokens();
}
 */
