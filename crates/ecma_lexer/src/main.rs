use TokenKind::*;

#[derive(Debug)]
enum TokenKind {
    CONST,
    IDENT,
    EOF,
    UNEXPECTED,
}


fn main() {
    let input = "const conrad";
    let mut tokens =  Vec::new();
    let mut chars = input.chars();

    loop {
        match chars.next().unwrap_or_default() {
            'c' => {
                match chars.next().unwrap_or_default() {
                    'o' => match chars.next().unwrap_or_default() {
                        'n' => match chars.next().unwrap_or_default() {
                            's' => match chars.next().unwrap_or_default() {
                                't' => tokens.push(CONST),
                                _ => tokens.push(IDENT),
                            },
                            _ => tokens.push(IDENT),
                        },
                        _ => tokens.push(IDENT),
                    },
                    _ => tokens.push(IDENT),
                }
            }
            '\x00' => {
                tokens.push(EOF);
                break;
            }
            _ => tokens.push(UNEXPECTED)
        }

    }

    println!("{:#?}", tokens);
}
