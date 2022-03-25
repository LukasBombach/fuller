#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Position {
  pub(crate) line: usize,
  pub(crate) col: usize,
}

impl Position {
  pub(crate) fn new() -> Position {
    Position { line: 1, col: 1 }
  }
}
