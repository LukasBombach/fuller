use std::str::Chars;

use self::TokenKind::*;

#[derive(Debug)]
struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TokenKind {
    // Keywords
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Export,
    Extends,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Return,
    Super,
    Switch,
    This,
    Throw,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield,

    // Future Keywords
    Enum,

    // Literals
    Null,
    True,
    False,
    Decimal,
    Exponential,
    Binary,
    Octal,
    Hexadecimal,
    Bigint,
    String,
    Template,

    // One-char tokens
    Semi,
    Comma,
    Dot,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Eq,
    Lt,
    Gt,
    Minus,
    And,
    Or,
    Plus,
    Star,
    Slash,
    Caret,
    Percent,

    // Control characters
    Zwnj,
    Zwj,
    Bom,

    // Whitespace
    Ht,
    Vt,
    Ff,
    Sp,
    Nbsp,
    Usp,

    // Line Terminators
    Lf,
    Cr,
    Ls,
    Ps,
}

const EOF_CHAR: char = '\0';

/// Peekable iterator over a char sequence.
struct Cursor<'a> {
    len: usize,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len: input.len(),
            chars: input.chars(),
        }
    }

    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn first(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        Some(c)
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek()) && !self.is_eof() {
            self.first();
        }
    }
}

impl Cursor<'_> {
    fn first_token(&mut self) -> Token {
        let first_char = self.first().unwrap();
        let token_kind = match first_char {
            'a'..='z' => self.keyword_or_ident(),
            'A'..='Z' => self.keyword_or_ident(),
        };
    }

    fn keyword_or_ident(&mut self) -> TokenKind {
        self.eat_while(is_ascii_letter);
    }
}

#[inline]
fn is_ascii_letter(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
}

fn first(input: &str) -> Token {
    Cursor::new(input).first_token()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn first_goalpost() {
        let input = "const val = true; if (val) { alert(val); }";

        assert_eq!(first(input), Token { kind: IF, len: 2 });
    }
}
