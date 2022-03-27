use match_keyword_derive::match_keyword;

match_keyword!();

fn main() {
    let input = "const";
    match_keyword!(SELECT * FROM posts WHERE id=1);
    /* (input, {
        "const" => Const,
        "let" => Let,
        "var" => Var,
    }); */
}
