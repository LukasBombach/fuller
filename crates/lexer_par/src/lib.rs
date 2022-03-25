mod cursor;
mod lexer;
mod token;

pub(crate) use crate::cursor::Cursor;
pub(crate) use crate::token::*;

/* impl Cursor<'_> {
    fn first_token(&mut self) -> Token {
        let first_char = self.first().unwrap();
        let token_kind = match first_char {
            'a'..='z' => self.keyword_or_ident(),
            'A'..='Z' => self.keyword_or_ident(),
        };
    }

    fn keyword_or_ident(&mut self) -> Token {
        self.eat_while(is_ascii_letter);
        let str = self.slice()

        Token::new()
    }

    fn is_keyword(str: &str) -> bool {

    }

} */

/* impl<'a> Cursor<'a> {
    fn slice(&mut self) -> &'a str {

    }
} */

/* #[inline]
fn is_ascii_letter(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
}

fn first(input: &str) -> Token {
    Cursor::new(input).first_token()
} */

#[cfg(test)]
mod tests {

    use super::TokenKind::*;
    use super::*;

    /* #[test]
    fn first_goalpost() {
        let input = "const val = true; if (val) { alert(val); }";

        assert_eq!(first(input), Token { kind: If, len: 2 });
    } */
}
