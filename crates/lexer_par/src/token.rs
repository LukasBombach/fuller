use crate::position::Position;

#[derive(Debug)]
pub(crate) struct Token {
  pub kind: TokenKind,
  pub start: Position,
  pub end: Position,
}

impl Token {
  pub(crate) fn new(kind: TokenKind, start: Position, end: Position) -> Token {
    Token { kind, start, end }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TokenKind {
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
  Percent,
  Conditional,
  BitNon,

  // Special Purpose
  Whitespace,
  Newline,
  Eof,
  Unknown,
}

// // Control characters
// Zwnj,
// Zwj,
// Bom,
//
// // Whitespace
// Ht,
// Vt,
// Ff,
// Sp,
// Nbsp,
// Usp,
//
// // Line Terminators
// Lf,
// Cr,
// Ls,
// Ps,
