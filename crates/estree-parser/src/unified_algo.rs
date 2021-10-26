extern crate ucd;
use ucd::Codepoint;

pub enum X {
  Const,
  Identifier,
  Unexpected,
}

pub fn algo() -> X {
  let mut input = "const a = 2;".chars();

  match input.next() {
    Some('c') => {
      for c in "onst".chars() {
        if Some(c) != input.next() {
          return X::Identifier;
        }
      }
      if let Some(c) = input.next() {
        if c.is_whitespace() || c == '[' || c == '{' {
          return X::Const;
        }
        if c.is_id_continue() {
          return X::Identifier;
        }
      }
      X::Unexpected
    }
    _ => X::Unexpected,
  }
}
