use lexer::tokenize;
use lexer::TokenKind;

fn main() {
    let mut tokens = tokenize("const x = 'foo';\nconst y = 'bar';");
    // let mut line: usize = 1;
    // let mut col: usize = 1;

    while let Some(token) = tokens.next() {
        println!("{:#?}", token.kind);

        // match token.kind {
        // TokenKind::Ident => {

        // }
        // }

        // col = col + token.len;
    }
}
