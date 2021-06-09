use logos::Logos;

pub use logos::Lexer;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

    #[regex("em|ex|ch|rem|vw|vh|vmin|vmax")]
    RelativeLength,

    #[regex("cm|mm|Q|in|pc|pt|px")]
    AbsoluteLength,

    #[regex("[+-]?[0-9]*[.]?[0-9]+(?:[eE][+-]?[0-9]+)?", priority = 2)]
    Number,

    #[regex("[-a-zA-Z_][a-zA-Z0-9_-]*")]
    Ident,

    #[token("{")]
    CurlyBracketOpen,

    #[token("}")]
    CurlyBracketClose,

    #[token("::")]
    DoubleColon,

    #[token(":")]
    Colon,

    #[token(";")]
    SemiColon,

    #[token(".")]
    Dot,

    #[token("#")]
    Hash,
}

pub fn get_lexer<'a>(source: &'a str) -> logos::Lexer<'a, Token>  {
    Token::lexer(source)
}
