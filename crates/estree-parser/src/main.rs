#![feature(iter_advance_by)]

pub mod scanner;
pub mod scanner_chars;
pub mod token;
pub mod token_chars;
pub mod token_string;

use scanner_chars::Scanner;

fn main() {
  let scanner = Scanner::new("   const myvar = 'my value';");

  scanner
    .into_iter()
    .for_each(|token| println!("{:#?}", token));
}
