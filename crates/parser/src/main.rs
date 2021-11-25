use lexer::tokenize;

fn main() {
    let tokens = tokenize("const x = 'foobar';");

    tokens
        .into_iter()
        .for_each(|token| println!("{:#?}", token));
}
