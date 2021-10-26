mod parse;

fn main() {
    let mut parser = parse::program::Parser::new("const a = 1;");

    let result = parser.scan_program();

    dbg!(result);
}
