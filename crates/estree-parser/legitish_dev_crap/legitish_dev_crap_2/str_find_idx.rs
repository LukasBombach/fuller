use std::ops::Range;

pub trait Source {
  fn find_idx<P>(&self, idx: usize, predicate: P) -> Option<usize>
  where
    Self: Sized,
    P: Fn(&str) -> bool;
}

impl Source for &str {
  fn find_idx<P>(&self, mut idx: usize, predicate: P) -> Option<usize>
  where
    Self: Sized,
    P: Fn(&str) -> bool,
  {
    while let Some(c) = self.get(Range {
      start: idx,
      end: idx + 1,
    }) {
      if predicate(c) {
        return Some(idx);
      }
      idx += 1;
    }
    None
  }
}
