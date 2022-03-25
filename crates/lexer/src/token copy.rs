use crate::segment::first_segment;
use crate::segment::SegmentKind;

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
  let mut cursor = Cursor::new(input);
  std::iter::from_fn(move || {
      if cursor.is_eof() {
          None
      } else {
          cursor.reset_len_consumed();
          Some(cursor.advance_token())
      }
  })
}

pub fn first_token(input: &str) -> Token {
  let token = first_segment(input);
  let len = token.len;

  match token.kind {
    SegmentKind::Eq => {
      let kind = TokenKind::Eq;
      Token { kind, len };
    }
    SegmentKind::Semi => {
      let kind = TokenKind::Semi;
      Token { kind, len };
    }
    SegmentKind::Ident => {
      let kind = keyword_or_identifier(&input[..token.len]);
      Token { kind, len };
    }
    SegmentKind::Literal { .. } => {
      let kind = TokenKind::Literal(&input[..token.len]);
      Token { kind, len };
    }
    SegmentKind::Whitespace => match &input[..token.len] {
      "\n" => pos.nl(),
      "\r" => pos,
      _ => pos.ff(1),
    },
    _ => {
      println!("Unhandled Token {:?} {:?}", token.kind, &input[..token.len]);
      pos.ff(token.len)
    }
  };
}

fn keyword_or_identifier(ident: &str) -> TokenKind {
  match ident {
    "break" => TokenKind::Keyword(Keyword::Break),
    "case" => TokenKind::Keyword(Keyword::Case),
    "catch" => TokenKind::Keyword(Keyword::Catch),
    "class" => TokenKind::Keyword(Keyword::Class),
    "const" => TokenKind::Keyword(Keyword::Const),
    "continue" => TokenKind::Keyword(Keyword::Continue),
    "debugger" => TokenKind::Keyword(Keyword::Debugger),
    "default" => TokenKind::Keyword(Keyword::Default),
    "delete" => TokenKind::Keyword(Keyword::Delete),
    "do" => TokenKind::Keyword(Keyword::Do),
    "else" => TokenKind::Keyword(Keyword::Else),
    "export" => TokenKind::Keyword(Keyword::Export),
    "extends" => TokenKind::Keyword(Keyword::Extends),
    "finally" => TokenKind::Keyword(Keyword::Finally),
    "for" => TokenKind::Keyword(Keyword::For),
    "function" => TokenKind::Keyword(Keyword::Function),
    "if" => TokenKind::Keyword(Keyword::If),
    "import" => TokenKind::Keyword(Keyword::Import),
    "in" => TokenKind::Keyword(Keyword::In),
    "instanceof" => TokenKind::Keyword(Keyword::Instanceof),
    "new" => TokenKind::Keyword(Keyword::New),
    "return" => TokenKind::Keyword(Keyword::Return),
    "super" => TokenKind::Keyword(Keyword::Super),
    "switch" => TokenKind::Keyword(Keyword::Switch),
    "this" => TokenKind::Keyword(Keyword::This),
    "throw" => TokenKind::Keyword(Keyword::Throw),
    "try" => TokenKind::Keyword(Keyword::Try),
    "typeof" => TokenKind::Keyword(Keyword::Typeof),
    "var" => TokenKind::Keyword(Keyword::Var),
    "void" => TokenKind::Keyword(Keyword::Void),
    "while" => TokenKind::Keyword(Keyword::While),
    "with" => TokenKind::Keyword(Keyword::With),
    "yield, " => TokenKind::Keyword(Keyword::Yield),
    value => TokenKind::Identifier(value),
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'a> {
  pub kind: TokenKind<'a>,
  pub len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind<'a> {
  Eq,
  Lt,
  Le,
  EqEq,
  Ne,
  Ge,
  Gt,
  AndAnd,
  OrOr,
  Not,
  Semi,
  Identifier(&'a str),
  Literal(&'a str),
  Keyword(Keyword),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
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
}
