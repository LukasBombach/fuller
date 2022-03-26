#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
  pub line: usize,
  pub col: usize,
}

impl Position {
  pub fn new() -> Position {
    Position { line: 1, col: 1 }
  }
}
