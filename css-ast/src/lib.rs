#[cfg(test)]
mod tests {
    use cssparser::{Parser, ParserInput};


    #[test]
    fn it_works() {
        let mut input = ParserInput::new(".red { color: red; }");
        let mut parser = Parser::new(&mut input);

        let transform = match parser.parse_entirely(|t| transform::parse(&context, t)) {
            Ok(result) => result,
            Err(..) => return Err(error::Error::Syntax),
        };
    
        assert_eq!(2 + 2, 4);
    }
}
