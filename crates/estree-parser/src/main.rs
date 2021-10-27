mod scanner;

use scanner::Scanner;

fn main() {
  let scanner = Scanner::new("const a = 1;");

  scanner
    .into_iter()
    .for_each(|token| println!("{:?}", token));
}
