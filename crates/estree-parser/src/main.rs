pub mod scanner;
pub mod token;

use scanner::Scanner;

fn main() {
  let scanner = Scanner::new("const myvar = 'my value';".chars());

  scanner
    .into_iter()
    .for_each(|token| println!("{:#?}", token));
}
