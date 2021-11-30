use lexer::segment::first_segment;
use lexer::segment::SegmentKind;

fn main() {
    let mut src = "const x = \"12\";\nconst y = \"12345678\";";
    let mut pos = Position { line: 1, col: 1 };

    while !src.is_empty() {
        let token = first_segment(src);

        pos = match token.kind {
            SegmentKind::Eq => {
                let kind = TokenKind::Eq;
                let (start, end) = span(pos, 0, token.len);
                let token = Token { kind, start, end };
                println!(
                    "{:9} {:?}",
                    format!("{:?}: {:2}-{:2}", start.line, start.col, end.col),
                    token.kind
                );
                end
            }
            SegmentKind::Semi => {
                let kind = TokenKind::Semi;
                let (start, end) = span(pos, 0, token.len);
                let token = Token { kind, start, end };
                println!(
                    "{:9} {:?}",
                    format!("{:?}: {:2}-{:2}", start.line, start.col, end.col),
                    token.kind
                );
                end
            }
            SegmentKind::Ident => {
                let kind = keyword_or_identifier(&src[..token.len]);
                let (start, end) = span(pos, 0, token.len);
                let token = Token { kind, start, end };
                println!(
                    "{:9} {:?}",
                    format!("{:?}: {:2}-{:2}", start.line, start.col, end.col),
                    token.kind
                );
                end
            }
            SegmentKind::Literal { .. } => {
                let kind = TokenKind::Literal(&src[..token.len]);
                let (start, end) = span(pos, 0, token.len);
                let token = Token { kind, start, end };
                println!(
                    "{:9} {:?}",
                    format!("{:?}: {:2}-{:2}", start.line, start.col, end.col),
                    token.kind
                );
                end
            }
            SegmentKind::Whitespace => match &src[..token.len] {
                "\n" => pos.nl(),
                "\r" => pos,
                _ => pos.ff(1),
            },
            _ => {
                println!("Unhandled Token {:?} {:?}", token.kind, &src[..token.len]);
                pos.ff(token.len)
            }
        };

        src = &src[token.len..];
    }
}

fn span(start: Position, lines: usize, cols: usize) -> (Position, Position) {
    let end = Position {
        line: start.line + lines,
        col: start.col + cols,
    };
    (start, end)
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
    kind: TokenKind<'a>,
    start: Position,
    end: Position,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    line: usize,
    col: usize,
}

impl Position {
    fn ff(self, len: usize) -> Position {
        Position {
            col: self.col + len,
            ..self
        }
    }

    fn nl(&self) -> Position {
        Position {
            line: self.line + 1,
            col: 1,
        }
    }
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
