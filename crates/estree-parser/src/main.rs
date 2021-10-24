extern crate ucd;
use ucd::Codepoint;

fn main() {
    // let source_code = 'let myvar = \'value\';';
    let source_code = "while (true) { console.log(); }";

    let mut chars = source_code.chars();

    match chars.next() {
        Some('{') => println!("BLOCK"),
        Some(';') => println!("EMPTY_STATEMENT"),
        Some('e') | Some('g') | Some('h') | Some('j') | Some('k') | Some('m') | Some('n')
        | Some('o') | Some('p') | Some('q') | Some('s') | Some('u') | Some('x') | Some('y')
        | Some('z') => {
            println!("IDENTIFIER")
        }
        _ => {}
    }
}
