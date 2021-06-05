use std::env::current_dir;
use std::boxed::Box;
use swc_common::{
  self,
  errors::{ColorConfig, Handler},
  sync::Lrc,
  FileName, SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};

fn something(path: &str) {

  let cwd = match current_dir() {
    Err(why) => panic!("couldn't gett the current dir: {}",  why),
    Ok(file) => file,
};

  // Create a path to the desired file
  let path = cwd.join(path);

  let cm: Lrc<SourceMap> = Default::default();
  let handler = Handler::with_emitter(true, false, Box::new(cm));

  // Real usage
   let fm = cm
       .load_file(&path)
       .expect("failed to load test.js");

  let lexer = Lexer::new(
      Syntax::Typescript(Default::default()),
      Default::default(),
      StringInput::from(&*fm),
      None,
  );

  let capturing = Capturing::new(lexer);

  let mut parser = Parser::new_from(capturing);

  for e in parser.take_errors() {
      e.into_diagnostic(&handler).emit();
  }

  let _module = parser
      .parse_typescript_module()
      .map_err(|e| e.into_diagnostic(&handler).emit())
      .expect("Failed to parse module.");

  println!("Tokens: {:?}", parser.input().take());
}

#[cfg(test)]
mod tests {
    use super::something;

    #[test]
    fn it_reads_a_file_from_the_disk() {
        let contents = something("assets/hello_world.ts");
        assert_eq!("console.log(\"hello world\");\n", contents);
    }
}

