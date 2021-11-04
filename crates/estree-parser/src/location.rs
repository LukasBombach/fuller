#[derive(Debug, Clone)]
pub struct Location {
  pub line: usize,
  pub column: usize,
}

impl Location {
  pub fn char(&mut self) {
    self.column += 1;
  }

  pub fn newline(&mut self) {
    self.column = 0;
    self.line += 1;
  }
}
