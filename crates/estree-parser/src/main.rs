mod source;

use source::Source;

fn main() {
  let source = Source::new("const myvar = 'my value';");

  source
    .into_iter()
    .for_each(|token| println!("{:#?}", token));
}
