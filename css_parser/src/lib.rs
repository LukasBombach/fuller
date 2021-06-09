use css_lexer::*;

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
}

impl<'a>  Parser<'a> {

    pub fn new(source: &'a str) -> Self {
        Parser {
            lexer: get_lexer(source),
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.token() {
            match token {
                _ => println!("_ {:?}", token),
            }
        }
    }

    fn token(&mut self) -> Option<Token> {
        self.lexer.next()
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn it_works() {
        let mut parser = Parser::new(".myClass { prop: val; }");
        parser.parse();
        assert_eq!(3, 1 +2);
    }
}
