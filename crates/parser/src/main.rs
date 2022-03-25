use lexer::tokenize;

fn main() {
    let mut src = "const x = \"12\";\nconst y = \"12345678\";";

    tokenize(src);
}