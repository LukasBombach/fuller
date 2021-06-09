use logos::Logos;
use css_lexer::Token;

pub fn parse(source: &str) {
    let lexer = Token::lexer(source);

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(3, 1 +2);
    }
}
