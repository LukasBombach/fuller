extern crate swc_common;
extern crate swc_ecma_parser;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName,  SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_ast::Module;

pub fn parse_typescript(src: &str) -> Module {
    let cm: Lrc<SourceMap> = Default::default();
    let handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false,
        Some(cm.clone()));

    let fm = cm.new_source_file(
        FileName::Custom("test.js".into()),
        src.into(),
    );

    let lexer = Lexer::new(
        // We want to parse TypeScript
        Syntax::Typescript(Default::default()),
        // JscTarget defaults to es5
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parser module");

    module
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let module = parse_typescript("function foo(str: string): number { return str.length; }");
        println!("module {:#?}", module);
        assert_eq!(1,1);
    }
}
