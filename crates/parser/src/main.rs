use lexer::first_token;

fn main() {
    let mut src = "const x = 'foo';\nconst y = 'bar';";
    let mut pos = Position { line: 1, col: 1 };

    while !src.is_empty() {
        let token = first_token(src);

        match token.kind {
            lexer::TokenKind::Eq => {
                let kind = TokenKind::Eq;
                let start = pos;
                let mut end = pos;
                end.ff(token.len);
                pos = end;
                let token = Token { kind, start, end };
                println!("{:?}", token);
            }
            lexer::TokenKind::Semi => {
                let kind = TokenKind::Semi;
                let start = pos;
                let mut end = pos;
                end.ff(token.len);
                pos = end;
                let token = Token { kind, start, end };
                println!("{:?}", token);
            }
            lexer::TokenKind::Ident => {
                let kind = keyword_or_identifier(&src[..token.len]);
                let start = pos;
                let mut end = pos;
                end.ff(token.len);
                pos = end;
                let token = Token { kind, start, end };
                println!("{:?}", token);
            }
            lexer::TokenKind::Whitespace => {
                let value = &src[..token.len];
                match value {
                    "\n" => pos.nl(),
                    "\r" => {}
                    _ => pos.ff(1),
                }
            }
            _ => {
                println!("{:?} {:?}", token.kind, &src[..token.len]);
            }
        }

        src = &src[token.len..];
    }
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
    fn ff(&mut self, len: usize) {
        self.col += len;
    }

    fn nl(&mut self) {
        self.line += 1;
        self.col = 1;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind<'a> {
    /* Expression-operator symbols. */
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

    /* Structural symbols */
    Semi,

    Identifier(&'a str),
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
