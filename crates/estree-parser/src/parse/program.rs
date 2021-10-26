extern crate ucd;
use std::str::Chars;
use std::string::String;
use std::vec::Vec;
use ucd::Codepoint;

type Input<'a> = std::iter::Peekable<std::str::Chars<'a>>;

#[derive(Debug, PartialEq)]
pub enum Node {
  Const,
  Identifier,
  Eoi,
}

#[derive(Debug, PartialEq)]
pub enum Token {
  Keyword,
  Identifier,
}

#[derive(Debug, PartialEq)]
pub struct UnexpectedChar(char);

#[derive(Debug, PartialEq)]
pub struct UnexpectedEoi();

pub struct Parser<'a> {
  pub input: Input<'a>,
  pub current_read: Vec<char>,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input: input.chars().peekable(),
      current_read: Vec::new(),
    }
  }

  pub fn scan_program(&mut self) -> Result<Node, UnexpectedChar> {
    // todo each follow up matcher should spawn an own iterator to work on that will also advance the input
    match self.next() {
      Some('c') => self.scan_const_declaration_from_c(),
      Some(c) => Err(UnexpectedChar(c)),
      None => Ok(Node::Eoi),
    }
  }

  fn scan_const_declaration_from_c(&mut self) -> Result<Node, UnexpectedChar> {
    for expected in "onst".chars() {
      if let Some(c) = self.next() {
        if c != expected {
          if c.is_id_continue() {
            return Ok(Node::Identifier);
          }
          return Err(UnexpectedChar(c));
        }
      }
      panic!("unexpected eoi");
    }

    let token = self.scan_keyword("onst");

    if let Some(c) = self.next() {
      if c.is_id_continue() {
        return Ok(Node::Identifier);
      }
      if !(c.is_whitespace() || c == '[' || c == '{') {
        return Err(UnexpectedChar(c));
      }
    } else {
      panic!("unexpected eoi");
    }

    let next_char = self.next_where(|c| !c.is_whitespace());

    Ok(Node::Const)
  }

  fn scan_keyword(&mut self, keyword: &str) -> Result<Token, UnexpectedChar> {
    for expected in keyword.chars() {
      if let Some(c) = self.next() {
        if c != expected {
          if c.is_id_continue() {
            return Ok(Token::Identifier);
          }
          return Err(UnexpectedChar(c));
        }
      }
      panic!("unexpected eoi");
    }
    Ok(Token::Keyword)
  }

  // todo use refernces instead of copying!
  fn next(&mut self) -> Option<char> {
    if let Some(c) = self.input.next() {
      self.current_read.push(c);
      return Some(c);
    }
    None
  }

  // todo use refernces instead of copying!
  fn next_where<P>(&mut self, predicate: P) -> Option<char>
  where
    Self: Sized,
    P: Fn(char) -> bool,
  {
    while let Some(c) = self.input.next() {
      self.current_read.push(c);
      if !predicate(c) {
        return Some(c);
      }
    }
    None
  }

  //fn peek(&mut self) -> Option<&char> {
  //  self.input.peek()
  //}
}
