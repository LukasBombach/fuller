mod cursor;

use crate::cursor::Cursor;

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

    use super::TokenKind::*;
    use super::*;

    #[test]
    fn first_goalpost() {
        let input = "const val = true; if (val) { alert(val); }";

        assert_eq!(first(input), Token { kind: If, len: 2 });
    }
}
