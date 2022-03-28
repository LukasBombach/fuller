/* use std::str::Chars;

pub struct Program;
pub struct Ident;
pub struct Whitespace;

pub struct Match<'a, S> {
    chars: Chars<'a>,
    _inner: S
}


// INITIALIZE
impl<'a> Match<'a, Program> {
    pub fn new(input: &'a str) -> Match<'a, Program> {
        Match { 
            chars: input.chars(),
            _inner: Program {} 
        }
    }
}

// MATCH STATE | Program
impl<'a> Match<'a, Program> {
    pub fn next(&mut self) -> Match<'a, Ident> {
        let c =   self.chars.next().unwrap_or_default();



        Match { 
            chars: self.chars,
            _inner: Ident {} 
        }
    }
}


// MATCH STATE | Ident
impl<'a> Match<'a, Ident> {
    pub fn next(&mut self) -> Match<'a, Whitespace> {
        Match { 
            chars: self.chars,
            _inner: Whitespace {} 
        }
    }
}

// MATCH STATE | Whitespace
impl<'a> Match<'a, Whitespace> {
    pub fn next(&mut self) -> Match<'a, Program> {
        Match { 
            chars: self.chars,
            _inner: Program {} 
        }
    }
} */