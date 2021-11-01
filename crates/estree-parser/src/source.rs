use std::ops::Range;

#[allow(dead_code)]
pub struct Source<'input> {
  pub input: &'input str,
  pub len: usize,
  pos: usize,
}

impl<'input> Source<'input> {
  pub fn new(input: &'input str) -> Self {
    Self {
      input,
      len: input.len(),
      pos: 0,
    }
  }
}

impl<'input> Source<'input> {
  pub fn find_idx<P>(&self, mut idx: usize, predicate: P) -> usize
  where
    Self: Sized,
    P: Fn(&str) -> bool,
  {
    while let Some(c) = self.input.get(Range {
      start: idx,
      end: idx + 1,
    }) {
      if predicate(c) {
        break;
      }
      idx += 1;
    }
    idx
  }
}

fn main() {
  let source = Source::new("const x = 'v';");
  let idx = source.find_idx(0, |c| c == " ");
  let val = &source.input[..idx];
  println!("{} `{}`", idx, val);
}
