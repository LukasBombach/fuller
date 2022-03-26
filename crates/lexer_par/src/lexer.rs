use crate::cursor::Cursor;
use crate::cursor::EOF_CHAR;
use crate::token::TokenKind::*;
use crate::token::*;

pub struct Lexer<'a> {
  cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    let cursor = Cursor::new(input);
    Lexer { cursor }
  }
}

impl<'a> Lexer<'a> {
  fn next_token(&mut self) -> Token {
    let start = self.cursor.pos.clone();
    loop {
      let next_char = self.cursor.next_char();
      let kind = match next_char {
        ' ' => {
          self.cursor.advance();
          continue;
        }
        '\t' => {
          self.cursor.advance();
          continue;
        }
        '\r' => {
          self.cursor.advance();
          continue;
        }
        '\n' => {
          self.cursor.newline();
          continue;
        }
        ';' => Semi,
        ',' => Comma,
        '.' => Dot,
        '(' => OpenParen,
        ')' => CloseParen,
        '{' => OpenBrace,
        '}' => CloseBrace,
        '[' => OpenBracket,
        ']' => CloseBracket,
        ':' => Colon,
        '=' => Eq,
        '<' => Lt,
        '>' => Gt,
        '-' => Minus,
        '&' => And,
        '|' => Or,
        '+' => Plus,
        '*' => Star,
        '/' => Slash,
        '%' => Percent,
        '?' => Conditional,
        '~' => BitNon,
        'a'..='z' => self.ident(),
        'A'..='Z' => self.ident(),
        EOF_CHAR => Eof,
        _ => Unknown,
      };
    }

    /* let start = self.cursor.pos.clone();
    let next_char = self.cursor.next_char();
    let kind = match next_char {
      ' ' => Whitespace,
      '\t' => Whitespace,
      '\r' => Whitespace,
      '\n' => Newline,
      ';' => Semi,
      ',' => Comma,
      '.' => Dot,
      '(' => OpenParen,
      ')' => CloseParen,
      '{' => OpenBrace,
      '}' => CloseBrace,
      '[' => OpenBracket,
      ']' => CloseBracket,
      ':' => Colon,
      '=' => Eq,
      '<' => Lt,
      '>' => Gt,
      '-' => Minus,
      '&' => And,
      '|' => Or,
      '+' => Plus,
      '*' => Star,
      '/' => Slash,
      '%' => Percent,
      '?' => Conditional,
      '~' => BitNon,
      'a'..='z' => self.ident(),
      'A'..='Z' => self.ident(),
      EOF_CHAR => Eof,
      _ => Unknown,
    };
    let end = self.cursor.pos.clone();
    Token::new(kind, start, end) */
  }
}

impl<'a> Lexer<'a> {
  fn ident(&mut self) -> TokenKind {}
}
