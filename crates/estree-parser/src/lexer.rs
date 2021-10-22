use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
  // Tokens can be literal strings, of any length.
  #[token("let")]
  VariableDeclaration,

  #[token("=")]
  Eq,

  #[regex("[a-zA-Z$_][a-zA-Z0-9$_]*")]
  Identifier,

  // Logos requires one token variant to handle errors,
  // it can be named anything you wish.
  #[error]
  // We can also use this variant to define whitespace,
  // or any other matches we wish to skip.
  #[regex(r"[ \t\n\f]+", logos::skip)]
  Error,
}
