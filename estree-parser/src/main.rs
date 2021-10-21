use std::str;

struct Node<'a> {
    r#type: &'a str,
    loc: Option<SourceLocation<'a>>,
}

struct SourceLocation<'a> {
    source: Option<&'a str>,
    start: Position,
    end: Position,
}

struct Position {
    line: u16,
    column: u16,
}

fn main() {
    println!("Hello, world!");
}
