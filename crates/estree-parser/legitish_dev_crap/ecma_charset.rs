pub trait EcmaCharset {
  fn is_id_start(&self) -> bool;
  fn is_id_continue(&self) -> bool;
  fn is_whitespace(&self) -> bool;
  fn is_quote(&self) -> bool;
  fn is_number(&self) -> bool;
}

impl EcmaCharset for char {
  fn is_id_start(&self) -> bool {
    matches!(self, 'a'..='z' | 'A'..='Z' | '_' | '$')
  }

  fn is_id_continue(&self) -> bool {
    matches!(self, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$' | '\u{005C}')
  }

  fn is_number(&self) -> bool {
    matches!(self, '0'..='9')
  }
  fn is_quote(&self) -> bool {
    matches!(self, '"' | '\'')
  }

  fn is_whitespace(&self) -> bool {
    matches!(
      self,
      '\u{0009}'
        | '\u{000B}'
        | '\u{000C}'
        | '\u{0020}'
        | '\u{00A0}'
        | '\u{FEFF}'
        | '\u{1680}'
        | '\u{2000}'
        | '\u{2001}'
        | '\u{2002}'
        | '\u{2003}'
        | '\u{2004}'
        | '\u{2005}'
        | '\u{2006}'
        | '\u{2007}'
        | '\u{2008}'
        | '\u{2009}'
        | '\u{200A}'
        | '\u{202F}'
        | '\u{205F}'
        | '\u{3000}'
    )
  }
}
