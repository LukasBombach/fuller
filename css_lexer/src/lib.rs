use logos::Logos;

pub use logos::Lexer;


#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Selector<'a> {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

    #[regex("[a-zA-Z0-9_-]+", |lex| lex.slice())]
    Tag(&'a str),

    #[regex("\\.[a-zA-Z0-9_-]+", |lex| lex.slice().strip_prefix("."))]
    Class(&'a str),

    #[regex("#[a-zA-Z0-9_-]+", |lex| lex.slice().strip_prefix("#"))]
    Id(&'a str),

    #[token(",")]
    Comma,

    #[token("{")]
    CurlyBracketOpen,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_selector() {
        let tokens: Vec<_> = Selector::lexer("div").collect();
        assert_eq!(tokens, &[Selector::Tag("div")]);
    }

    #[test]
    fn test_class_selector() {
        let tokens: Vec<_> = Selector::lexer(".my_class").collect();
        assert_eq!(tokens, &[Selector::Class("my_class")]);

    }

    #[test]
    fn test_id_selector() {
        let tokens: Vec<_> = Selector::lexer("#my_id").collect();
        assert_eq!(tokens, &[Selector::Id("my_id")]);

    }

}
