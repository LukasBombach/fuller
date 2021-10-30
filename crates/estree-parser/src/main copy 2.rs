use std::fmt::Debug;
use std::ops::Range;

struct Source<'src> {
  pub input: &'src str,
  pub len: usize,
}

impl<'src> Source<'src> {
  pub fn new(input: &'src str) -> Self {
    Self {
      input,
      len: input.len(),
    }
  }

  #[inline]
  pub unsafe fn slice_unchecked(&self, range: Range<usize>) -> &[u8] {
    debug_assert!(
      range.start <= self.len && range.end <= self.len,
      "Reading out of bounds {:?} for {}!",
      range,
      self.len
    );

    self.get_unchecked(range)
  }
}

fn main() {
  let input = "hello world";

  let read = input.get(11..12);

  dbg!(input.len());
  dbg!(read);
}
