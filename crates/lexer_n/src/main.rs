use match_keyword::match_keyword;

fn main() {
    let input = "const";
    match_keyword!(input {
        "const" => Const,
        "let" => Let,
        "var" => Var,
    });
}
