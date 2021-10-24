extern crate ucd;
use std::str::Chars;

fn get_inner_ast_type(source_code: Chars) {
    match chars.next() {
        Some('{') => println!("BLOCK"),
        Some(';') => println!("EMPTY_STATEMENT"),

        Some('d') => match chars.next() {
            Some('o') => println!("MAYBE DO"),
            _ => println!("IDENTIFIER"),
        },

        // async requires multipeek
        // Some('a') => {}

        // NO KEYWORD
        // Some('e') | Some('g') | Some('h') | Some('j') | Some('k') | Some('m') | Some('n')
        // | Some('o') | Some('p') | Some('q') | Some('s') | Some('u') | Some('x') | Some('y')
        // | Some('z') => {
        //     println!("IDENTIFIER")
        // }
        _ => println!("IDENTIFIER"),
    }
}

fn main() {
    // let source_code = 'let myvar = \'value\';';
    let source_code = ";{ while (true) { console.log(); } }";

    let mut chars = source_code.chars();

    match chars.next() {
        Some('{') => println!("BLOCK"),
        Some(';') => println!("EMPTY_STATEMENT"),

        Some('d') => match chars.next() {
            Some('o') => println!("MAYBE DO"),
            _ => println!("IDENTIFIER"),
        },

        // async requires multipeek
        // Some('a') => {}

        // NO KEYWORD
        // Some('e') | Some('g') | Some('h') | Some('j') | Some('k') | Some('m') | Some('n')
        // | Some('o') | Some('p') | Some('q') | Some('s') | Some('u') | Some('x') | Some('y')
        // | Some('z') => {
        //     println!("IDENTIFIER")
        // }
        _ => println!("IDENTIFIER"),
    }
}
