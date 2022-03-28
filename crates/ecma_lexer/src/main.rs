use TokenKind::*;

enum TokenKind {
    EOF,
}



fn main() {
    let input = "const foo = true;";
    let mut tokens =  Vec::new<TokenKind>();

    let mut chars = input.chars();

    loop {
        let c = chars.next().unwrap_or_else(|| {
            tokens.push(EOF);
            break   
            
             });


        match c {
            'c' => {}
        }
    }
}
