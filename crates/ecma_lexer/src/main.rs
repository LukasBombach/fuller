use self::Keyword::*;
use match_keyword::match_keyword;

enum Keyword {
    Const,
    Ident,
    Whitespace,
    Eq,
    True,
    Semi,
}

fn main() {
    let input = "const conrad = true;";
    match_keyword!(match input {
        "const" => Const,
        "const" => Const,
        "=" => Eq,
        "true" => True,
        ";" => Semi,
        _ => Ident,
    });
}
