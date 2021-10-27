extern crate ucd;
use ucd::Codepoint;

/* fn unified_algo() {
  let line_terminator = ["<LF>", "<CR>", "<LS>", "<PS>"];
  let whitespace_str = [
    "<TAB>", "<VT>", "<FF>", "<SP>", "<NBSP>", "<ZWNBSP>", "<USP>",
  ];
  let whitespace = [
    "<LF>", "<CR>", "<LS>", "<PS>", "<TAB>", "<VT>", "<FF>", "<SP>", "<NBSP>", "<ZWNBSP>", "<USP>",
  ];

  let keyword = "const";
  let can_be_followed_by = [
    '[', '{', "<LF>", "<CR>", "<LS>", "<PS>", "<TAB>", "<VT>", "<FF>", "<SP>", "<NBSP>",
    "<ZWNBSP>", "<USP>",
  ];

  let const_match = ['c', 'o', 'n', 's', 't'].iter();
}
 */

enum X {
  Identifier,
  NoMatch,
}

fn algo() -> X {
  let mut input = "const a = 2;".chars();

  match input.next() {
    Some('c') => {
      for c in "onst".chars() {
        if Some(c) != input.next() {
          return X::Identifier;
        }
      }
      if 
    }
    _ => {
      return X::NoMatch;
    }
  }
}
