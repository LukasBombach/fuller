use lexer::first_token;

fn main() {
    let mut src = "const x = 'foo';\nconst y = 'bar';";

    while !src.is_empty() {
        let token = first_token(src);
        println!("{:?} {:?}", token.kind, &src[..token.len]);
        src = &src[token.len..];
    }
}
