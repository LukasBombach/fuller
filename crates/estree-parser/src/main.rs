mod scanner_2;

use scanner_2::Scanner;

fn main() {
  let scanner = Scanner::new("const myvar = 'my value';");

  //   let start = scanner.next_non_whitespace_idx(0);
  //   let end = scanner.next_whitespace_idx(start);

  //println!("len, {}", scanner.len);
  //println!("{}-{}", start, end);

  scanner
    .into_iter()
    .for_each(|token| println!("{:#?}", token));
}
