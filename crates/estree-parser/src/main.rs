pub mod ecma_charset;
pub mod scanner;
use scanner::Scanner;

fn main() {
  let scanner = Scanner::new("const myConst = 123;");

  scanner
    .into_iter()
    .for_each(|token| println!("{:?}", token));
}
