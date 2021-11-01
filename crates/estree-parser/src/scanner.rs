use crate::token::Token;

pub struct Location {
  pub line: usize,
  pub column: usize,
}

pub struct Scanner<'src> {
  source: &'src str,
  loc: Location,
  len: usize,
  pos: usize,
}

impl<'src> Scanner<'src> {
  pub fn new(source: &'src str) -> Self {
    Self {
      source,
      loc: Location { line: 0, column: 0 },
      len: source.len(),
      pos: 0,
    }
  }
}

impl<'src> Iterator for Scanner<'src> {
  type Item = Token<'src>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pos == self.len {
      return None;
    }

    // todo this is akward
    self.skip_whitespace();

    // todo use pointers, not values
    let next_token = match self.source.as_bytes()[self.pos] as char {
      '=' => self.token(Token::Eq, 1),
      ';' => self.token(Token::Semicolon, 1),
      '\n' => self.newline(1),
      '\r' => self.newline(1),
      c if matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '$') => return self.identifier(),
      // c if matches!(c, '"' | '\'') => return self.literal(c),
      // c if matches!(c, '0'..='9') => return self.number(c),
      // todo c matches unicode ID_Start
      c => self.token(Token::Unknown(c), 1),
    };

    next_token
  }
}

impl<'src> Scanner<'src> {
  // todo this seems like a highly ineffective implementation
  #[inline]
  fn find_offset<P>(&self, predicate: P) -> usize
  where
    Self: Sized,
    P: Fn(char) -> bool,
  {
    let source_from_pos = &self.source[self.pos..];
    match source_from_pos.find(predicate) {
      Some(offset) => offset,
      None => source_from_pos.len(),
    }
  }

  // todo this is even worse than find_offset
  #[inline]
  fn find_position<P>(&self, predicate: P) -> usize
  where
    Self: Sized,
    P: Fn(char) -> bool,
  {
    let offset = self.find_offset(predicate);
    self.pos + offset
  }
}

impl<'src> Scanner<'src> {
  #[inline]
  fn skip_whitespace(&mut self) {
    let idx = self.find_offset(|c| c != ' ');
    self.pos += idx;
    self.loc.column += idx;
  }
}

impl<'src> Scanner<'src> {
  #[inline]
  fn identifier(&mut self) -> Option<Token<'src>> {
    let end = self.find_position(|c| c == ' ');
    let val = &self.source[self.pos..end];
    return self.token(Token::Identifier(val), end - self.pos);
  }

  #[inline]
  fn token(&mut self, token: Token<'src>, len: usize) -> Option<Token<'src>> {
    self.pos += len;
    self.loc.column += len;
    Some(token)
  }

  #[inline]
  fn newline(&mut self, len: usize) -> Option<Token<'src>> {
    self.pos += len;
    self.loc.column = 0;
    self.loc.line += 1;
    Some(Token::Newline)
  }
}
