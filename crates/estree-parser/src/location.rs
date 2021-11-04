#[derive(Debug, Clone)]
pub struct Location(pub usize, pub usize);

impl Location {
  pub fn char(&mut self) {
    self.1 += 1;
  }

  pub fn newline(&mut self) {
    self.1 = 0;
    self.0 += 1;
  }
}
